#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::view::View // Import to get access to the time uniform

// Adjust these constants to change the pattern's look
const STARBURST_DENSITY: f32 = 15.0; // How many "points" the starburst has
const PULSE_SPEED: f32 = 40.0;       // How fast the colors change and pulse
const COLOR_MIX_POWER: f32 = 3.0;   // Controls the intensity of color blending

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

// Bevy provides a View uniform which includes time!
@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Center the UV coordinates so (0,0) is the middle of the mesh
    let centered_uv = mesh.uv - vec2<f32>(0.5, 0.5);

    // Calculate distance from the center (0.0 to approx 0.707 for corners)
    let distance = length(centered_uv);

    // Calculate the angle around the center (-PI to PI)
    let angle = atan2(centered_uv.y, centered_uv.x);

    // Use time to animate the pattern
    let time = globals.time;

    // Create a base pattern using angle, distance, and time
    // This will create radiating "spokes" that shift and pulse
    let pattern_factor = sin(angle * STARBURST_DENSITY + distance * 5.0 + time * PULSE_SPEED);

    // Use the pattern_factor to create vibrant colors
    // We'll map the -1 to 1 range of sin to different color components
    let r = (sin(pattern_factor * 1.5 + time) * 0.5) + 0.5;
    let g = (sin(pattern_factor * 2.0 + time + 2.0) * 0.5) + 0.5;
    let b = (sin(pattern_factor * 2.5 + time + 4.0) * 0.5) + 0.5;

    // Create a dynamic color based on the pattern
    let dynamic_color = vec4<f32>(r, g, b, 1.0);

    // Blend the dynamic color with the original material_color
    // The COLOR_MIX_POWER makes the blending more intense/sharper
    var final_color = mix(material_color, dynamic_color, pow(pattern_factor * 0.5 + 0.5, COLOR_MIX_POWER));

    // You can still apply a global multiplier for overall brightness/alpha
    final_color *= vec4<f32>(1.0, 1.0, 1.0, 0.8); // Adjust overall transparency

    return final_color;
}