use bevy::prelude::*;

use crate::module::*;

pub struct NoiseModule;

impl Plugin for NoiseModule {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_module.run_if(
                on_event::<SpawnModuleInternalEvent>.and(run_if_module(ModuleClass::Noise)),
            ),
        )
        .add_systems(Update, noise_system);
    }
}

fn spawn_module(
    mut ev_spawn: EventReader<SpawnModuleInternalEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shadermaterials: ResMut<Assets<CustomMaterial>>,
) {
    // Spawn the noise module entities here
    println!("Spawning Noise Module");

    for ev in ev_spawn.read() {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(BOXWIDTH / 8.0, BOXHEIGHT / 8.0))),
            //MeshMaterial2d(colormaterials.add(Color::srgb(0.0, 1.0, 0.0))),
            MeshMaterial2d(shadermaterials.add(CustomMaterial {
                color: LinearRgba::GREEN,
            })),
            Transform::default(),
            HDirection::Right,
            VDirection::Up,
            FirstPassEntity {
                spriteid: ev.spriteid,
            },
            ev.layer.clone(),
        ));
    }
}

fn noise_system(time: Res<Time>) {}
