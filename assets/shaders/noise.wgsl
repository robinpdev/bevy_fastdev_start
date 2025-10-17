#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::view::View // Import to get access to the time uniform

// Adjust these constants to change the pattern's look
const STARBURST_DENSITY: f32 = 15.0; // How many "points" the starburst has
const PULSE_SPEED: f32 = 40.0;       // How fast the colors change and pulse
const COLOR_MIX_POWER: f32 = 3.0;   // Controls the intensity of color blending

// Bevy provides a View uniform which includes time!
@group(0) @binding(0) var<uniform> view: View;

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

@group(2) @binding(1) var<uniform> width: f32;
@group(2) @binding(2) var<uniform> height: f32;



// MIT License. © Stefan Gustavson, Munrocket
//
fn mod289(x: vec4f) -> vec4f { return x - floor(x * (1. / 289.)) * 289.; }
fn perm4(x: vec4f) -> vec4f { return mod289(((x * 34.) + 1.) * x); }

fn noise3(p: vec3f) -> f32 {
    let a = floor(p);
    var d: vec3f = p - a;
    d = d * d * (3. - 2. * d);

    let b = a.xxyy + vec4f(0., 1., 0., 1.);
    let k1 = perm4(b.xyxy);
    let k2 = perm4(k1.xyxy + b.zzww);

    let c = k2 + a.zzzz;
    let k3 = perm4(c);
    let k4 = perm4(c + 1.);

    let o1 = fract(k3 * (1. / 41.));
    let o2 = fract(k4 * (1. / 41.));

    let o3 = o2 * d.z + o1 * (1. - d.z);
    let o4 = o3.yw * d.x + o3.xz * (1. - d.x);

    return o4.y * d.y + o4.x * (1. - d.y);
}


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Center the UV coordinates so (0,0) is the middle of the mesh
    let resolution = view.viewport.zw;
    let uv = (mesh.uv - vec2<f32>(0.5, 0.5)) * vec2<f32>(width, height) / 400.0;

    // Use time to animate the pattern
    let time = globals.time;

    // Blend the dynamic color with the original material_color
    // The COLOR_MIX_POWER makes the blending more intense/sharper
    let noise_value = noise3(vec3<f32>(uv * 100.0, time));
    var final_color = vec4<f32>(noise_value, noise_value, noise_value, 1.0);

    // You can still apply a global multiplier for overall brightness/alpha
    final_color *= vec4<f32>(1.0, 1.0, 1.0, 1.0); // Adjust overall transparency

    return final_color;
}