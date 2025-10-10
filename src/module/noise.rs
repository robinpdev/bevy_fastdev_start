use bevy::prelude::*;

use crate::module::*;

use bevy::{reflect::TypePath, render::render_resource::AsBindGroup};
use bevy::{sprite_render::Material2dPlugin};


use bevy::{
    shader::ShaderRef,
    sprite_render::{Material2d},
};

pub struct NoiseModule;

impl Plugin for NoiseModule {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_module).run_if(
                on_message::<SpawnModuleInternalEvent>.and(run_if_module::<SpawnModuleEvent>(ModuleClass::Noise)),),
            
        )
        .add_plugins(Material2dPlugin::<NoiseMaterial>::default());
    }
}

// fn resize_rect(

// )

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NoiseMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/noise.wgsl";

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for NoiseMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    // fn alpha_mode(&self) -> AlphaMode2d {
    //     AlphaMode2d::Mask(0.5)
    // }
}

fn spawn_module(
    mut ev_spawn: MessageReader<SpawnModuleInternalEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shadermaterials: ResMut<Assets<NoiseMaterial>>,
) {
    // Spawn the noise module entities here
    println!("Spawning Noise Module");

    for ev in ev_spawn.read() {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(BOXWIDTH, BOXHEIGHT))),
            //MeshMaterial2d(colormaterials.add(Color::srgb(0.0, 1.0, 0.0))),
            MeshMaterial2d(shadermaterials.add(NoiseMaterial {
                color: LinearRgba::GREEN,
            })),
            Transform::default(),
            FirstPassEntity {
                module_id: ev.module_id,
            },
            ev.layer.clone(),
        ));
    }
}
