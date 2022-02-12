use super::{
    ast::*,
    context::Context,
    error::{Error, ErrorKind},
    Parser, Result, Span,
};
use crate::{
    AddressSpace, Binding, Block, BuiltIn, Constant, Expression, GlobalVariable, Handle,
    Interpolation, LocalVariable, ResourceBinding, ScalarKind, ShaderStage, SwizzleComponent, Type,
    TypeInner, VectorSize,
};

pub struct VarDeclaration<'a, 'key> {
    pub qualifiers: &'a mut TypeQualifiers<'key>,
    pub ty: Handle<Type>,
    pub name: Option<String>,
    pub init: Option<Handle<Constant>>,
    pub meta: Span,
}

/// Information about a builtin used in [`add_builtin`](Parser::add_builtin).
struct BuiltInData {
    /// The type of the builtin.
    inner: TypeInner,
    /// The associated builtin class.
    builtin: BuiltIn,
    /// Whether the builtin can be written to or not.
    mutable: bool,
    /// The storage used for the builtin.
    storage: StorageQualifier,
}

pub enum GlobalOrConstant {
    Global(Handle<GlobalVariable>),
    Constant(Handle<Constant>),
}

impl Parser {
    /// Adds a builtin and returns a variable reference to it
    fn add_builtin(
        &mut self,
        ctx: &mut Context,
        body: &mut Block,
        name: &str,
        data: BuiltInData,
        meta: Span,
    ) -> Option<VariableReference> {
        let ty = self.module.types.insert(
            Type {
                name: None,
                inner: data.inner,
            },
            meta,
        );

        let handle = self.module.global_variables.append(
            GlobalVariable {
                name: Some(name.into()),
                space: AddressSpace::Private,
                binding: None,
                ty,
                init: None,
            },
            meta,
        );

        let idx = self.entry_args.len();
        self.entry_args.push(EntryArg {
            name: None,
            binding: Binding::BuiltIn(data.builtin),
            handle,
            storage: data.storage,
        });

        self.global_variables.push((
            name.into(),
            GlobalLookup {
                kind: GlobalLookupKind::Variable(handle),
                entry_arg: Some(idx),
                mutable: data.mutable,
            },
        ));

        let expr = ctx.add_expression(Expression::GlobalVariable(handle), meta, body);
        ctx.lookup_global_var_exps.insert(
            name.into(),
            VariableReference {
                expr,
                load: true,
                mutable: data.mutable,
                constant: None,
                entry_arg: Some(idx),
            },
        );

        ctx.lookup_global_var(name)
    }

    pub(crate) fn lookup_variable(
        &mut self,
        ctx: &mut Context,
        body: &mut Block,
        name: &str,
        meta: Span,
    ) -> Option<VariableReference> {
        if let Some(local_var) = ctx.lookup_local_var(name) {
            return Some(local_var);
        }
        if let Some(global_var) = ctx.lookup_global_var(name) {
            return Some(global_var);
        }

        let data = match name {
            "gl_Position" => BuiltInData {
                inner: TypeInner::Vector {
                    size: VectorSize::Quad,
                    kind: ScalarKind::Float,
                    width: 4,
                },
                builtin: BuiltIn::Position,
                mutable: true,
                storage: StorageQualifier::Output,
            },
            "gl_FragCoord" => BuiltInData {
                inner: TypeInner::Vector {
                    size: VectorSize::Quad,
                    kind: ScalarKind::Float,
                    width: 4,
                },
                builtin: BuiltIn::Position,
                mutable: false,
                storage: StorageQualifier::Input,
            },
            "gl_GlobalInvocationID"
            | "gl_NumWorkGroups"
            | "gl_WorkGroupSize"
            | "gl_WorkGroupID"
            | "gl_LocalInvocationID" => BuiltInData {
                inner: TypeInner::Vector {
                    size: VectorSize::Tri,
                    kind: ScalarKind::Uint,
                    width: 4,
                },
                builtin: match name {
                    "gl_GlobalInvocationID" => BuiltIn::GlobalInvocationId,
                    "gl_NumWorkGroups" => BuiltIn::NumWorkGroups,
                    "gl_WorkGroupSize" => BuiltIn::WorkGroupSize,
                    "gl_WorkGroupID" => BuiltIn::WorkGroupId,
                    "gl_LocalInvocationID" => BuiltIn::LocalInvocationId,
                    _ => unreachable!(),
                },
                mutable: false,
                storage: StorageQualifier::Input,
            },
            "gl_FrontFacing" => BuiltInData {
                inner: TypeInner::Scalar {
                    kind: ScalarKind::Bool,
                    width: crate::BOOL_WIDTH,
                },
                builtin: BuiltIn::FrontFacing,
                mutable: false,
                storage: StorageQualifier::Input,
            },
            "gl_PointSize" | "gl_FragDepth" => BuiltInData {
                inner: TypeInner::Scalar {
                    kind: ScalarKind::Float,
                    width: 4,
                },
                builtin: match name {
                    "gl_PointSize" => BuiltIn::PointSize,
                    "gl_FragDepth" => BuiltIn::FragDepth,
                    _ => unreachable!(),
                },
                mutable: true,
                storage: StorageQualifier::Output,
            },
            "gl_ClipDistance" | "gl_CullDistance" => {
                let base = self.module.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Scalar {
                            kind: ScalarKind::Float,
                            width: 4,
                        },
                    },
                    meta,
                );

                BuiltInData {
                    inner: TypeInner::Array {
                        base,
                        size: crate::ArraySize::Dynamic,
                        stride: 4,
                    },
                    builtin: match name {
                        "gl_ClipDistance" => BuiltIn::PointSize,
                        "gl_CullDistance" => BuiltIn::FragDepth,
                        _ => unreachable!(),
                    },
                    mutable: self.meta.stage == ShaderStage::Vertex,
                    storage: StorageQualifier::Output,
                }
            }
            _ => {
                let builtin = match name {
                    "gl_BaseVertex" => BuiltIn::BaseVertex,
                    "gl_BaseInstance" => BuiltIn::BaseInstance,
                    "gl_PrimitiveID" => BuiltIn::PrimitiveIndex,
                    "gl_InstanceIndex" => BuiltIn::InstanceIndex,
                    "gl_VertexIndex" => BuiltIn::VertexIndex,
                    "gl_SampleID" => BuiltIn::SampleIndex,
                    "gl_LocalInvocationIndex" => BuiltIn::LocalInvocationIndex,
                    _ => return None,
                };

                BuiltInData {
                    inner: TypeInner::Scalar {
                        kind: ScalarKind::Uint,
                        width: 4,
                    },
                    builtin,
                    mutable: false,
                    storage: StorageQualifier::Input,
                }
            }
        };

        self.add_builtin(ctx, body, name, data, meta)
    }

    pub(crate) fn field_selection(
        &mut self,
        ctx: &mut Context,
        lhs: bool,
        body: &mut Block,
        expression: Handle<Expression>,
        name: &str,
        meta: Span,
    ) -> Result<Handle<Expression>> {
        let (ty, is_pointer) = match *self.resolve_type(ctx, expression, meta)? {
            TypeInner::Pointer { base, .. } => (&self.module.types[base].inner, true),
            ref ty => (ty, false),
        };
        match *ty {
            TypeInner::Struct { ref members, .. } => {
                let index = members
                    .iter()
                    .position(|m| m.name == Some(name.into()))
                    .ok_or_else(|| Error {
                        kind: ErrorKind::UnknownField(name.into()),
                        meta,
                    })?;
                Ok(ctx.add_expression(
                    Expression::AccessIndex {
                        base: expression,
                        index: index as u32,
                    },
                    meta,
                    body,
                ))
            }
            // swizzles (xyzw, rgba, stpq)
            TypeInner::Vector { size, .. } => {
                let check_swizzle_components = |comps: &str| {
                    name.chars()
                        .map(|c| {
                            comps
                                .find(c)
                                .filter(|i| *i < size as usize)
                                .map(|i| SwizzleComponent::from_index(i as u32))
                        })
                        .collect::<Option<Vec<SwizzleComponent>>>()
                };

                let components = check_swizzle_components("xyzw")
                    .or_else(|| check_swizzle_components("rgba"))
                    .or_else(|| check_swizzle_components("stpq"));

                if let Some(components) = components {
                    if lhs {
                        let not_unique = (1..components.len())
                            .any(|i| components[i..].contains(&components[i - 1]));
                        if not_unique {
                            self.errors.push(Error {
                                kind:
                                ErrorKind::SemanticError(
                                format!(
                                    "swizzle cannot have duplicate components in left-hand-side expression for \"{:?}\"",
                                    name
                                )
                                .into(),
                            ),
                                meta ,
                            })
                        }
                    }

                    let mut pattern = [SwizzleComponent::X; 4];
                    for (pat, component) in pattern.iter_mut().zip(&components) {
                        *pat = *component;
                    }

                    // flatten nested swizzles (vec.zyx.xy.x => vec.z)
                    let mut expression = expression;
                    if let Expression::Swizzle {
                        size: _,
                        vector,
                        pattern: ref src_pattern,
                    } = ctx[expression]
                    {
                        expression = vector;
                        for pat in &mut pattern {
                            *pat = src_pattern[pat.index() as usize];
                        }
                    }

                    let size = match components.len() {
                        1 => {
                            // only single element swizzle, like pos.y, just return that component.
                            if lhs {
                                // Because of possible nested swizzles, like pos.xy.x, we have to unwrap the potential load expr.
                                if let Expression::Load { ref pointer } = ctx[expression] {
                                    expression = *pointer;
                                }
                            }
                            return Ok(ctx.add_expression(
                                Expression::AccessIndex {
                                    base: expression,
                                    index: pattern[0].index(),
                                },
                                meta,
                                body,
                            ));
                        }
                        2 => VectorSize::Bi,
                        3 => VectorSize::Tri,
                        4 => VectorSize::Quad,
                        _ => {
                            self.errors.push(Error {
                                kind: ErrorKind::SemanticError(
                                    format!("Bad swizzle size for \"{:?}\"", name).into(),
                                ),
                                meta,
                            });

                            VectorSize::Quad
                        }
                    };

                    if is_pointer {
                        // NOTE: for lhs expression, this extra load ends up as an unused expr, because the
                        // assignment will extract the pointer and use it directly anyway. Unfortunately we
                        // need it for validation to pass, as swizzles cannot operate on pointer values.
                        expression = ctx.add_expression(
                            Expression::Load {
                                pointer: expression,
                            },
                            meta,
                            body,
                        );
                    }

                    Ok(ctx.add_expression(
                        Expression::Swizzle {
                            size,
                            vector: expression,
                            pattern,
                        },
                        meta,
                        body,
                    ))
                } else {
                    Err(Error {
                        kind: ErrorKind::SemanticError(
                            format!("Invalid swizzle for vector \"{}\"", name).into(),
                        ),
                        meta,
                    })
                }
            }
            _ => Err(Error {
                kind: ErrorKind::SemanticError(
                    format!("Can't lookup field on this type \"{}\"", name).into(),
                ),
                meta,
            }),
        }
    }

    pub(crate) fn add_global_var(
        &mut self,
        ctx: &mut Context,
        body: &mut Block,
        VarDeclaration {
            qualifiers,
            ty,
            name,
            init,
            meta,
        }: VarDeclaration,
    ) -> Result<GlobalOrConstant> {
        let storage = qualifiers.storage.0;
        let (ret, lookup) = match storage {
            StorageQualifier::Input | StorageQualifier::Output => {
                let input = storage == StorageQualifier::Input;
                // TODO: glslang seems to use a counter for variables without
                // explicit location (even if that causes collisions)
                let location = qualifiers
                    .uint_layout_qualifier("location", &mut self.errors)
                    .unwrap_or(0);
                let interpolation = qualifiers.interpolation.take().map(|(i, _)| i).or_else(|| {
                    let kind = self.module.types[ty].inner.scalar_kind()?;
                    Some(match kind {
                        ScalarKind::Float => Interpolation::Perspective,
                        _ => Interpolation::Flat,
                    })
                });
                let sampling = qualifiers.sampling.take().map(|(s, _)| s);

                let handle = self.module.global_variables.append(
                    GlobalVariable {
                        name: name.clone(),
                        space: AddressSpace::Private,
                        binding: None,
                        ty,
                        init,
                    },
                    meta,
                );

                let idx = self.entry_args.len();
                self.entry_args.push(EntryArg {
                    name: name.clone(),
                    binding: Binding::Location {
                        location,
                        interpolation,
                        sampling,
                    },
                    handle,
                    storage,
                });

                let lookup = GlobalLookup {
                    kind: GlobalLookupKind::Variable(handle),
                    entry_arg: Some(idx),
                    mutable: !input,
                };

                (GlobalOrConstant::Global(handle), lookup)
            }
            StorageQualifier::Const => {
                let init = init.ok_or_else(|| Error {
                    kind: ErrorKind::SemanticError("const values must have an initializer".into()),
                    meta,
                })?;

                let lookup = GlobalLookup {
                    kind: GlobalLookupKind::Constant(init, ty),
                    entry_arg: None,
                    mutable: false,
                };

                (GlobalOrConstant::Constant(init), lookup)
            }
            StorageQualifier::AddressSpace(mut space) => {
                match space {
                    AddressSpace::Storage { ref mut access } => {
                        if let Some((restricted_access, _)) = qualifiers.storage_acess.take() {
                            access.remove(restricted_access);
                        }
                    }
                    AddressSpace::Uniform => {
                        if let TypeInner::Image { .. } | TypeInner::Sampler { .. } =
                            self.module.types[ty].inner
                        {
                            space = AddressSpace::Handle
                        } else if qualifiers
                            .none_layout_qualifier("push_constant", &mut self.errors)
                        {
                            space = AddressSpace::PushConstant
                        }
                    }
                    AddressSpace::Function => space = AddressSpace::Private,
                    _ => {}
                };

                let binding = match space {
                    AddressSpace::Uniform | AddressSpace::Storage { .. } | AddressSpace::Handle => {
                        let binding = qualifiers.uint_layout_qualifier("binding", &mut self.errors);
                        if binding.is_none() {
                            self.errors.push(Error {
                                kind: ErrorKind::SemanticError(
                                    "uniform/buffer blocks require layout(binding=X)".into(),
                                ),
                                meta,
                            });
                        }
                        let set = qualifiers.uint_layout_qualifier("set", &mut self.errors);
                        binding.map(|binding| ResourceBinding {
                            group: set.unwrap_or(0),
                            binding,
                        })
                    }
                    _ => None,
                };

                let handle = self.module.global_variables.append(
                    GlobalVariable {
                        name: name.clone(),
                        space,
                        binding,
                        ty,
                        init,
                    },
                    meta,
                );

                let lookup = GlobalLookup {
                    kind: GlobalLookupKind::Variable(handle),
                    entry_arg: None,
                    mutable: true,
                };

                (GlobalOrConstant::Global(handle), lookup)
            }
        };

        if let Some(name) = name {
            ctx.add_global(self, &name, lookup, body);

            self.global_variables.push((name, lookup));
        }

        qualifiers.unused_errors(&mut self.errors);

        Ok(ret)
    }

    pub(crate) fn add_local_var(
        &mut self,
        ctx: &mut Context,
        body: &mut Block,
        decl: VarDeclaration,
    ) -> Result<Handle<Expression>> {
        #[cfg(feature = "glsl-validate")]
        if let Some(ref name) = decl.name {
            if ctx.lookup_local_var_current_scope(name).is_some() {
                self.errors.push(Error {
                    kind: ErrorKind::VariableAlreadyDeclared(name.clone()),
                    meta: decl.meta,
                })
            }
        }

        let storage = decl.qualifiers.storage;
        let mutable = match storage.0 {
            StorageQualifier::AddressSpace(AddressSpace::Function) => true,
            StorageQualifier::Const => false,
            _ => {
                self.errors.push(Error {
                    kind: ErrorKind::SemanticError("Locals cannot have a storage qualifier".into()),
                    meta: storage.1,
                });
                true
            }
        };

        let handle = ctx.locals.append(
            LocalVariable {
                name: decl.name.clone(),
                ty: decl.ty,
                init: decl.init,
            },
            decl.meta,
        );
        let expr = ctx.add_expression(Expression::LocalVariable(handle), decl.meta, body);

        if let Some(name) = decl.name {
            ctx.add_local_var(name, expr, mutable);
        }

        decl.qualifiers.unused_errors(&mut self.errors);

        Ok(expr)
    }
}