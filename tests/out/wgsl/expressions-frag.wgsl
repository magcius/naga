struct BST {
    data: i32,
}

struct FragmentOutput {
    @location(0) o_color: vec4<f32>,
}

var<private> global: f32;
var<private> o_color: vec4<f32>;

fn testBinOpVecFloat(a: vec4<f32>, b: f32) {
    var a_1: vec4<f32>;
    var b_1: f32;
    var v: vec4<f32>;

    a_1 = a;
    b_1 = b;
    let _e5 = a_1;
    v = (_e5 * 2.0);
    let _e8 = a_1;
    v = (_e8 / vec4<f32>(2.0));
    let _e12 = a_1;
    v = (_e12 + vec4<f32>(2.0));
    let _e16 = a_1;
    v = (_e16 - vec4<f32>(2.0));
    return;
}

fn testBinOpFloatVec(a_2: vec4<f32>, b_2: f32) {
    var a_3: vec4<f32>;
    var b_3: f32;
    var v_1: vec4<f32>;

    a_3 = a_2;
    b_3 = b_2;
    let _e5 = a_3;
    let _e6 = b_3;
    v_1 = (_e5 * _e6);
    let _e8 = a_3;
    let _e9 = b_3;
    v_1 = (_e8 / vec4<f32>(_e9));
    let _e12 = a_3;
    let _e13 = b_3;
    v_1 = (_e12 + vec4<f32>(_e13));
    let _e16 = a_3;
    let _e17 = b_3;
    v_1 = (_e16 - vec4<f32>(_e17));
    return;
}

fn testBinOpIVecInt(a_4: vec4<i32>, b_4: i32) {
    var a_5: vec4<i32>;
    var b_5: i32;
    var v_2: vec4<i32>;

    a_5 = a_4;
    b_5 = b_4;
    let _e5 = a_5;
    let _e6 = b_5;
    v_2 = (_e5 * _e6);
    let _e8 = a_5;
    let _e9 = b_5;
    v_2 = (_e8 / vec4<i32>(_e9));
    let _e12 = a_5;
    let _e13 = b_5;
    v_2 = (_e12 + vec4<i32>(_e13));
    let _e16 = a_5;
    let _e17 = b_5;
    v_2 = (_e16 - vec4<i32>(_e17));
    let _e20 = a_5;
    let _e21 = b_5;
    v_2 = (_e20 & vec4<i32>(_e21));
    let _e24 = a_5;
    let _e25 = b_5;
    v_2 = (_e24 | vec4<i32>(_e25));
    let _e28 = a_5;
    let _e29 = b_5;
    v_2 = (_e28 ^ vec4<i32>(_e29));
    let _e32 = a_5;
    let _e33 = b_5;
    v_2 = (_e32 >> vec4<u32>(u32(_e33)));
    let _e37 = a_5;
    let _e38 = b_5;
    v_2 = (_e37 << vec4<u32>(u32(_e38)));
    return;
}

fn testBinOpIntIVec(a_6: i32, b_6: vec4<i32>) {
    var a_7: i32;
    var b_7: vec4<i32>;
    var v_3: vec4<i32>;

    a_7 = a_6;
    b_7 = b_6;
    let _e5 = a_7;
    let _e6 = b_7;
    v_3 = (_e5 * _e6);
    let _e8 = a_7;
    let _e9 = b_7;
    v_3 = (vec4<i32>(_e8) + _e9);
    let _e12 = a_7;
    let _e13 = b_7;
    v_3 = (vec4<i32>(_e12) - _e13);
    let _e16 = a_7;
    let _e17 = b_7;
    v_3 = (vec4<i32>(_e16) & _e17);
    let _e20 = a_7;
    let _e21 = b_7;
    v_3 = (vec4<i32>(_e20) | _e21);
    let _e24 = a_7;
    let _e25 = b_7;
    v_3 = (vec4<i32>(_e24) ^ _e25);
    return;
}

fn testBinOpUVecUint(a_8: vec4<u32>, b_8: u32) {
    var a_9: vec4<u32>;
    var b_9: u32;
    var v_4: vec4<u32>;

    a_9 = a_8;
    b_9 = b_8;
    let _e5 = a_9;
    let _e6 = b_9;
    v_4 = (_e5 * _e6);
    let _e8 = a_9;
    let _e9 = b_9;
    v_4 = (_e8 / vec4<u32>(_e9));
    let _e12 = a_9;
    let _e13 = b_9;
    v_4 = (_e12 + vec4<u32>(_e13));
    let _e16 = a_9;
    let _e17 = b_9;
    v_4 = (_e16 - vec4<u32>(_e17));
    let _e20 = a_9;
    let _e21 = b_9;
    v_4 = (_e20 & vec4<u32>(_e21));
    let _e24 = a_9;
    let _e25 = b_9;
    v_4 = (_e24 | vec4<u32>(_e25));
    let _e28 = a_9;
    let _e29 = b_9;
    v_4 = (_e28 ^ vec4<u32>(_e29));
    let _e32 = a_9;
    let _e33 = b_9;
    v_4 = (_e32 >> vec4<u32>(_e33));
    let _e36 = a_9;
    let _e37 = b_9;
    v_4 = (_e36 << vec4<u32>(_e37));
    return;
}

fn testBinOpUintUVec(a_10: u32, b_10: vec4<u32>) {
    var a_11: u32;
    var b_11: vec4<u32>;
    var v_5: vec4<u32>;

    a_11 = a_10;
    b_11 = b_10;
    let _e5 = a_11;
    let _e6 = b_11;
    v_5 = (_e5 * _e6);
    let _e8 = a_11;
    let _e9 = b_11;
    v_5 = (vec4<u32>(_e8) + _e9);
    let _e12 = a_11;
    let _e13 = b_11;
    v_5 = (vec4<u32>(_e12) - _e13);
    let _e16 = a_11;
    let _e17 = b_11;
    v_5 = (vec4<u32>(_e16) & _e17);
    let _e20 = a_11;
    let _e21 = b_11;
    v_5 = (vec4<u32>(_e20) | _e21);
    let _e24 = a_11;
    let _e25 = b_11;
    v_5 = (vec4<u32>(_e24) ^ _e25);
    return;
}

fn testBinOpMatMat(a_12: mat3x3<f32>, b_12: mat3x3<f32>) {
    var a_13: mat3x3<f32>;
    var b_13: mat3x3<f32>;
    var v_6: mat3x3<f32>;
    var c: bool;

    a_13 = a_12;
    b_13 = b_12;
    let _e6 = a_13;
    let _e7 = b_13;
    v_6 = mat3x3<f32>((_e6[0] / _e7[0]), (_e6[1] / _e7[1]), (_e6[2] / _e7[2]));
    let _e18 = a_13;
    let _e19 = b_13;
    v_6 = (_e18 * _e19);
    let _e21 = a_13;
    let _e22 = b_13;
    v_6 = (_e21 + _e22);
    let _e24 = a_13;
    let _e25 = b_13;
    v_6 = (_e24 - _e25);
    let _e27 = a_13;
    let _e28 = b_13;
    c = (all((_e27[2] == _e28[2])) && (all((_e27[1] == _e28[1])) && all((_e27[0] == _e28[0]))));
    let _e43 = a_13;
    let _e44 = b_13;
    c = (any((_e43[2] != _e44[2])) || (any((_e43[1] != _e44[1])) || any((_e43[0] != _e44[0]))));
    return;
}

fn testBinOpMatFloat(a_14: f32, b_14: mat3x3<f32>) {
    var a_15: f32;
    var b_15: mat3x3<f32>;
    var v_7: mat3x3<f32>;

    a_15 = a_14;
    b_15 = b_14;
    let _e5 = a_15;
    let _e6 = b_15;
    let _e7 = vec3<f32>(_e5);
    v_7 = mat3x3<f32>((_e7 / _e6[0]), (_e7 / _e6[1]), (_e7 / _e6[2]));
    let _e15 = a_15;
    let _e16 = b_15;
    v_7 = (_e15 * _e16);
    let _e18 = a_15;
    let _e19 = b_15;
    let _e20 = vec3<f32>(_e18);
    v_7 = mat3x3<f32>((_e20 + _e19[0]), (_e20 + _e19[1]), (_e20 + _e19[2]));
    let _e28 = a_15;
    let _e29 = b_15;
    let _e30 = vec3<f32>(_e28);
    v_7 = mat3x3<f32>((_e30 - _e29[0]), (_e30 - _e29[1]), (_e30 - _e29[2]));
    let _e38 = b_15;
    let _e39 = a_15;
    let _e40 = vec3<f32>(_e39);
    v_7 = mat3x3<f32>((_e38[0] / _e40), (_e38[1] / _e40), (_e38[2] / _e40));
    let _e48 = b_15;
    let _e49 = a_15;
    v_7 = (_e48 * _e49);
    let _e51 = b_15;
    let _e52 = a_15;
    let _e53 = vec3<f32>(_e52);
    v_7 = mat3x3<f32>((_e51[0] + _e53), (_e51[1] + _e53), (_e51[2] + _e53));
    let _e61 = b_15;
    let _e62 = a_15;
    let _e63 = vec3<f32>(_e62);
    v_7 = mat3x3<f32>((_e61[0] - _e63), (_e61[1] - _e63), (_e61[2] - _e63));
    return;
}

fn testUnaryOpMat(a_16: mat3x3<f32>) {
    var a_17: mat3x3<f32>;
    var v_8: mat3x3<f32>;

    a_17 = a_16;
    let _e3 = a_17;
    v_8 = -(_e3);
    let _e5 = a_17;
    let _e7 = vec3<f32>(1.0);
    let _e9 = (_e5 - mat3x3<f32>(_e7, _e7, _e7));
    a_17 = _e9;
    v_8 = _e9;
    let _e10 = a_17;
    let _e12 = vec3<f32>(1.0);
    a_17 = (_e10 - mat3x3<f32>(_e12, _e12, _e12));
    v_8 = _e10;
    return;
}

fn testStructConstructor() {
    var tree: BST = BST(1);

}

fn testNonScalarToScalarConstructor() {
    var f: f32 = 1.0;

}

fn testArrayConstructor() {
    var tree_1: array<f32,1u> = array<f32,1u>(0.0);

}

fn testFreestandingConstructor() {
    return;
}

fn testNonImplicitCastVectorCast() {
    var a_18: u32 = 1u;
    var b_16: vec4<i32>;

    let _e3 = a_18;
    b_16 = vec4<i32>(i32(_e3));
    return;
}

fn privatePointer(a_19: ptr<function, f32>) {
    return;
}

fn main_1() {
    var local: f32;

    let _e3 = global;
    local = _e3;
    privatePointer((&local));
    let _e5 = local;
    global = _e5;
    let _e6 = o_color;
    let _e9 = vec4<f32>(1.0);
    o_color.x = _e9.x;
    o_color.y = _e9.y;
    o_color.z = _e9.z;
    o_color.w = _e9.w;
    return;
}

@stage(fragment) 
fn main() -> FragmentOutput {
    main_1();
    let _e5 = o_color;
    return FragmentOutput(_e5);
}