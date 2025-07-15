use bevy::prelude::*;

use bevy::{
    reflect::TypePath,
    render::render_resource::{AsBindGroup},
};

use bevy::{
    render::render_resource::{ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

#[derive(Component)]
pub enum HDirection {
    Left,
    Right,
}

#[derive(Component)]
pub enum VDirection {
    Up,
    Down,
}

pub const SPEED: f32 = 10.0;
pub const RADIUS: f32 = 100.0;

pub const BOXWIDTH: f32 = 700.0;
pub const BOXHEIGHT: f32 = 512.0;

/// Marker component for things we spawn once and never despawn
#[derive(Component)]
pub struct Immortal;


#[derive(Component)]
pub struct FirstPassEntity;

#[derive(Component)]
pub struct ModuleWin;

/// Boilerplate for setting up a basic restarting architecture:
/// The two states (Re)starting and Running
#[derive(States, Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    /// Nothing happens in this state other than moving immediately to the Running state
    #[default]
    Restarting,
    Startup,
    // When we enter this state, we run any user-defined setup code (what would normally live in Startup / Prestartup for example)
    // When we exit this state we tear down anything that was spawned
    Running,
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/test_shader_2d.wgsl";

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Mask(0.5)
    }
}
