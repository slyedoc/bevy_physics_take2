struct AabbMaterial {
    color: vec4<f32>;
    width: f32;
};
[[group(1), binding(0)]]
var<uniform> material: AabbMaterial;

[[stage(fragment)]]
fn fragment() -> [[location(0)]] vec4<f32> {
    return material.color;
}
