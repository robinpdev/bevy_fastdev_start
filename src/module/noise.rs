use bevy::prelude::*;

use crate::module::*;

use bevy::{reflect::TypePath, render::render_resource::AsBindGroup};
use bevy::{prelude::*, sprite::Material2dPlugin, window::PrimaryWindow};


use bevy::{
    render::render_resource::ShaderRef,
    sprite::{AlphaMode2d, Material2d},
};

pub struct NoiseModule;

impl Plugin for NoiseModule {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_module.run_if(
                on_event::<SpawnModuleInternalEvent>.and(run_if_module(ModuleClass::Noise)),
            ),
        )
        .add_plugins(Material2dPlugin::<NoiseMaterial>::default())

        .add_systems(Update, noise_system);
    }
}

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
    mut ev_spawn: EventReader<SpawnModuleInternalEvent>,
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
                spriteid: ev.spriteid,
            },
            ev.layer.clone(),
        ));
    }
}

fn noise_system(time: Res<Time>) {}
