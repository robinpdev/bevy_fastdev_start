mod common;
mod module;
mod ui;

use bevy_egui::EguiContext;
use common::*;

// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy::diagnostic::LogDiagnosticsPlugin;

use bevy_simple_subsecond_system::prelude::*;
use iyes_perf_ui::prelude::*;

use bevy_persistent::prelude::*;
use bevy_persistent_windows::prelude::*;

use bevy::{prelude::*, sprite::Material2dPlugin, window::PrimaryWindow};
use std::path::Path;

#[hot]
fn main() {
    let mut bevyapp = App::new();

    let mut default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: None,
        ..default()
    });

    // Conditionally add the AssetPlugin for Linux
    #[cfg(all(target_os = "linux"))]
    {
        print!("ADDING WATCHER PLUGIN");
        default_plugins = default_plugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        });
    }

    bevyapp.add_plugins(default_plugins.build());

    bevyapp.world_mut().spawn((
        PrimaryWindow,
        PersistentWindowBundle {
            window: Window {
                title: "I am the primary window!".to_owned(),
                ..Default::default()
            },
            state: Persistent::<WindowState>::builder()
                .name("primary window state")
                .format(StorageFormat::Toml)
                .path("./primary-window.toml")
                .default(WindowState::windowed(1280, 720))
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to create the persistent primary window state"),
        },
    ));

    bevyapp
        .insert_resource(ClearColor(Color::srgba(0.2, 0.2, 0.2, 1.0)))
        .add_plugins((
            // default_plugins,
            // LogDiagnosticsPlugin::default(),
            bevy_framepace::FramepacePlugin,
        ))
        .add_plugins(PersistentWindowsPlugin)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PerfUiPlugin)
        .add_plugins(ui::BumpUiPlugin)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        // .edit_schedule(Update, |schedule| {
        //     schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        // })
        .init_state::<AppState>()
        .add_systems(Startup, restart)
        .add_systems(OnEnter(AppState::Restarting), restart)
        .add_systems(OnEnter(AppState::Startup), setup)
        .add_systems(OnExit(AppState::Running), teardown)
        .add_systems(PreUpdate, trigger_restart)
        .add_systems(PreStartup, spawn_immortals)
        .add_plugins(module::PongModulePlugin);

    bevyapp.run();
}

/// Boilerplate for setting up a basic restarting architecture:
/// Moves the state into AppState::Running so that the OnEnter(AppState::Running) system is called
fn restart(mut next_state: ResMut<NextState<AppState>>) {
    println!("restart!");
    next_state.set(AppState::Startup);
}

/// Boilerplate for setting up a basic restarting architecture:
/// Moves the state into AppState::Running so that the OnEnter(AppState::Running) system is called
fn trigger_restart(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        println!("user triggered restart");
        next_state.set(AppState::Restarting);
    } else if input.just_pressed(KeyCode::KeyT) {
        println!("user triggered FULL restart");
        app_exit_events.send(bevy::app::AppExit::Success);
    }
}

/// Code that is actually! run once on startup of your program
/// You can spawn entities with the Immortal component (above) here and they will not be removed when restarting
fn spawn_immortals(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    println!("immortal");
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(30.0);
}

fn get_component_names(world: &World, entity: Entity) -> Option<Vec<String>> {
    world
        .inspect_entity(entity)
        .ok() // Convert Result<EntityInspector, EntityDoesNotExistError> to Option<EntityInspector>
        .map(|entity_inspector| {
            entity_inspector
                .map(|component_info| component_info.name().to_string()) // Get the name and convert to String
                .collect::<Vec<String>>() // Collect into a Vec<String>
        })
}

/// User-defined teardown code can live here
/// If you kill all the Windows it will quit the app, so we use Without<PrimaryWindow> here
/// We also don't despawn the "immortals"
fn teardown(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            Without<bevy::window::PrimaryWindow>,
            Without<bevy::picking::pointer::PointerInteraction>,
            Without<bevy::ecs::observer::Observer>,
            Without<bevy::window::Monitor>,
            Without<Immortal>,
            Without<ComputedNode>,
            Without<EguiContext>,
        ),
    >,
    world: &World
) {
    // if you want to see what components that entities about to be despawned have

    for entity in query.iter() {
        if let Some(component_names) = get_component_names(world, entity) {
            println!("Component names for entity {:?}: {:?}", entity, component_names);
        } else {
            // This branch is now reached if the entity doesn't exist.
            println!("Entity {:?} does not exist.", entity);
        }
    }

    println!("teardown!");
    for entity in query.iter() {
        // Drain to clear the vec
        commands.entity(entity).despawn();
        println!("Despawned entity: {:?}", entity);
    }
}

/// Runs each time the scene is (re)started
/// Sets up a circle that gets rendered to a texture and then shown on the main context
#[hot]
fn setup(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    println!("setup!");

    // main camera
    commands.spawn(Camera2d);

    // create a simple Perf UI
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            values_col_width: 32.0,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));

    next_state.set(AppState::Running);
}
