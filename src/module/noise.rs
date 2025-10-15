use bevy::ecs::relationship::RelationshipSourceCollection;
use bevy::prelude::*;

use crate::module::*;

use bevy::sprite_render::Material2dPlugin;
use bevy::{reflect::TypePath, render::render_resource::AsBindGroup};

use bevy::{shader::ShaderRef, sprite_render::Material2d};

pub struct NoiseModule;

impl Plugin for NoiseModule {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<NoiseMaterial>::default())
            .add_systems(OnEnter(AppState::Startup), setup)
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut spawnerconfig: ResMut<ModuleSpawnerConfig>
){
    let eid = commands.spawn((ModuleSpawner{
            class: ModuleClass::Noise
        },))
        .observe(spawn_noise_module)
        .observe(resize_surface)
        .id();

    spawnerconfig.observers.insert(ModuleClass::Noise, vec![eid]);
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

pub fn spawn_noise_module(
    spawn: On<SpawnModuleInternalEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shadermaterials: ResMut<Assets<NoiseMaterial>>,
) {
    if spawn.moduleclass != ModuleClass::Noise { return };
    // Spawn the noise module entities here
    println!("Spawning Noise Module");

    let shadersurface : Entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1., 1.))),
        //MeshMaterial2d(colormaterials.add(Color::srgb(0.0, 1.0, 0.0))),
        MeshMaterial2d(shadermaterials.add(NoiseMaterial {
            color: LinearRgba::GREEN,
        })),
        Transform::default().with_scale(Vec3::new(BOXWIDTH, BOXHEIGHT, 1.0)),
        FirstPassEntity {
            module_id: spawn.root_id,
        },
        spawn.layer.clone(),
    )).id();

    commands.entity(spawn.root_id).add_child(shadersurface);
}

fn resize_surface(
    resize: On<ResizeModule>,
    mut surfaces: Query<&mut Transform, With<Mesh2d>>,
    roots: Query<&Children, With<ModuleWin>>,
){
    println!("resizing surface...");
    if let Ok(rootchildren) = roots.get(resize.entity){
        for child in rootchildren.iter(){
            if let Ok(mut transform) = surfaces.get_mut(child){
                let newscale = Vec3::new(resize.width, resize.height, 1.0);
                transform.scale = newscale;
            }
        }
    }
}
