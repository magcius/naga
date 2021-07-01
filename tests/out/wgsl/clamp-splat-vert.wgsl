struct VertexOutput {
    [[builtin(position)]] member: vec4<f32>;
};

var<private> a_pos1: vec2<f32>;
var<private> gl_Position: vec4<f32>;

fn main1() {
    let _e2: vec2<f32> = a_pos1;
    gl_Position = vec4<f32>(clamp(_e2, vec2<f32>(0.0), vec2<f32>(1.0)), 0.0, 1.0);
    return;
}

[[stage(vertex)]]
fn main([[location(0), interpolate(perspective)]] a_pos: vec2<f32>) -> VertexOutput {
    a_pos1 = a_pos;
    main1();
    let _e3: vec4<f32> = gl_Position;
    return VertexOutput(_e3);
}
