use bevy::prelude::*;

use crate::module::*;

pub struct PongModule;

impl Plugin for PongModule {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_module.run_if(
                on_event::<SpawnModuleInternalEvent>.and(run_if_module::<SpawnModuleInternalEvent>(ModuleClass::Pong)),
            ),
        )
        .add_systems(Update, pong_system.run_if(in_state(AppState::Running)));
    }
}

fn spawn_module(
    mut ev_spawn: EventReader<SpawnModuleInternalEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shadermaterials: ResMut<Assets<CustomMaterial>>,
) {
    // Spawn the noise module entities here
    println!("Spawning Pong Module");

    for ev in ev_spawn.read() {
        //first pass circle mesh
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(RADIUS))),
            //MeshMaterial2d(colormaterials.add(Color::srgb(0.0, 1.0, 0.0))),
            MeshMaterial2d(shadermaterials.add(CustomMaterial {
                color: LinearRgba::RED,
            })),
            Transform::default(),
            HDirection::Right,
            VDirection::Up,
            FirstPassEntity{ModuleId: ev.moduleID},
            ev.layer.clone(),
        ));
    }
}

/// Rotates the inner cube (first pass)
#[hot]
fn pong_system(
    mut query: Query<(&mut Transform, &mut VDirection, &mut HDirection, &FirstPassEntity)>,
    modules: Query<&Sprite, With<ModuleWin>>,
) {
    // for mut transform in &mut query {
    //     transform.rotate_x(1.5 * time.delta_secs());
    //     transform.rotate_z(0.4 * time.delta_secs());
    // }

    for (mut pos, mut vdir, mut hdir, fpe) in &mut query {
        let sprite = modules.get(fpe.ModuleId).unwrap();
        let boxwidth = sprite.custom_size.unwrap().x;
        let boxheight = sprite.custom_size.unwrap().y;

        // println!("size: {:?} x {:?}", boxwidth, boxheight);

        match *vdir {
            VDirection::Up => pos.translation.y += SPEED,
            VDirection::Down => pos.translation.y -= SPEED,
        }

        match *hdir {
            HDirection::Left => pos.translation.x -= SPEED,
            HDirection::Right => pos.translation.x += SPEED,
        }

        if pos.translation.x + RADIUS > boxwidth / 2 as f32 {
            *hdir = HDirection::Left
        } else if pos.translation.x - RADIUS < -boxwidth / 2 as f32 {
            *hdir = HDirection::Right
        }

        if pos.translation.y + RADIUS > boxheight / 2 as f32 {
            *vdir = VDirection::Down
        } else if pos.translation.y - RADIUS < -boxheight / 2 as f32 {
            *vdir = VDirection::Up
        }
    }
}
