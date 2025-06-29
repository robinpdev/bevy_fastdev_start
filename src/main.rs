use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::ecs::component::ComponentId;
use bevy::ecs::component::Components;
use bevy::prelude::*;
use bevy::render::texture;
use bevy_mod_imgui::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

use bevy::{
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    window::{PresentMode, WindowTheme},
};

use bevy::ecs::schedule::ExecutorKind;

#[derive(Resource)]
struct ImguiState {
    demo_window_open: bool,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgba(0.2, 0.2, 0.2, 1.0)))
        .insert_resource(ImguiState {
            demo_window_open: true,
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am the window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (500., 300.).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin::default(),
            bevy_framepace::FramepacePlugin,
        ))
        .edit_schedule(Update, |schedule| {
            schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        })
        .init_state::<AppState>()
        .add_systems(Startup, restart)
        .add_systems(PreUpdate, trigger_restart)
        .add_systems(PreStartup, spawn_immortals)
        .add_systems(OnEnter(AppState::Restarting), restart)
        .add_systems(OnEnter(AppState::Running), setup)
        .add_systems(OnExit(AppState::Running), teardown)
        .add_plugins(bevy_mod_imgui::ImguiPlugin {
            ini_filename: Some("hello-world.ini".into()),
            font_oversample_h: 2,
            font_oversample_v: 2,
            ..default()
        })
        .init_resource::<MySpawnedEntities>() // Add your resource here
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(
            Update,
            (greet, rotator_system).run_if(in_state(AppState::Running)),
        )
        .add_systems(Update, imgui_example_ui.run_if(in_state(AppState::Running)));

    app.run();
}

#[derive(Component)]
struct FirstPassEntity;

/// Boilerplate for setting up a basic restarting architecture:
/// The two states (Re)starting and Running
#[derive(States, Default, Debug, Clone, Hash, Eq, PartialEq)]
enum AppState {
    /// Nothing happens in this state other than moving immediately to the Running state
    #[default]
    Restarting,
    // When we enter this state, we run any user-defined setup code (what would normally live in Startup / Prestartup for example)
    // When we exit this state we tear down anything that was spawned
    Running,
}

/// Boilerplate for setting up a basic restarting architecture:
/// Moves the state into AppState::Running so that the OnEnter(AppState::Running) system is called
fn restart(mut next_state: ResMut<NextState<AppState>>) {
    println!("restart!");
    next_state.set(AppState::Running);
}

/// Boilerplate for setting up a basic restarting architecture:
/// Moves the state into AppState::Running so that the OnEnter(AppState::Running) system is called
fn trigger_restart(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::KeyR) {
        println!("user triggered restart");
        next_state.set(AppState::Restarting);
    }
}

/// Marker component for things we spawn once and never despawn
#[derive(Component)]
struct Immortal;

/// We spawn these once in PreStartup and never need to despawn this
fn spawn_immortals(
    mut commands: Commands,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
) {
    println!("immortal");
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(30.0);
}

#[derive(Resource, Default)]
struct MySpawnedEntities {
    entities: Vec<Entity>,
}

/// User-defined teardown code can live here
/// If you kill all the Windows it will quit the app, so we use Without<PrimaryWindow> here
/// We also don't despawn the "immortals"
fn teardown(
    mut commands: Commands,
    mut my_spawned_entities: ResMut<MySpawnedEntities>,
) {
    println!("teardown!");
    for entity in my_spawned_entities.entities.drain(..) {
        // Drain to clear the vec
        commands.entity(entity).despawn();
        println!("Despawned entity: {:?}", entity);
    }
}

#[hot]
fn greet(time: Res<Time>) {
    info_once!(
        "Hello there from a hotpatched system! Try changing this string while the app is running! Patched at t = {} s",
        time.elapsed_secs()
    );
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut smaterials: ResMut<Assets<StandardMaterial>>,
    mut colormaterials: ResMut<Assets<ColorMaterial>>,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut images: ResMut<Assets<Image>>,
    mut my_spawned_entities: ResMut<MySpawnedEntities>,
) {
    println!("setup!");

    // rendered texture
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    //first pass circle mesh
    let circlemesh = meshes.add(Circle::new(50.0));
    let emeshfp = commands
        .spawn((
            Mesh2d(circlemesh),
            MeshMaterial2d(colormaterials.add(Color::srgb(0.0, 1.0, 0.0))),
            Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            FirstPassEntity,
            first_pass_layer.clone(),
        ))
        .id();

    //first pass camera
    let ecamerafp = commands
        .spawn((
            Camera2d::default(),
            Camera {
                target: image_handle.clone().into(),
                clear_color: Color::WHITE.into(),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)).looking_at(Vec3::ZERO, Vec3::Y),
            first_pass_layer,
        ))
        .id();

    // //main plane
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
    //     MeshMaterial3d(smaterials.add(Color::srgb(0.3, 0.5, 0.3))),
    // ));
    //main cube
    let esprite = commands
        .spawn(Sprite::from_image(image_handle.clone()))
        .id();
    // main light

    // first pass light
    // commands.spawn((
    //     PointLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(4.0, 8.0, 4.0),
    //     RenderLayers::layer(1)
    // ));

    // main camera
    let ecammain = commands.spawn(Camera2d).id();

    my_spawned_entities.entities.push(emeshfp);
    my_spawned_entities.entities.push(ecamerafp);
    my_spawned_entities.entities.push(esprite);
    my_spawned_entities.entities.push(ecammain);

    // commands.spawn((
    //     Transform::from_xyz(1.7, 1.7, 2.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
    //     Camera3d::default(),
    // ));
}

/// Rotates the inner cube (first pass)
#[hot]
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassEntity>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_secs());
        transform.rotate_z(0.4 * time.delta_secs());
    }
}

fn imgui_example_ui(mut context: NonSendMut<ImguiContext>, mut state: ResMut<ImguiState>) {
    let ui = context.ui();
    let window = ui.window("Hello mf world");
    window
        .size([300.0, 100.0], imgui::Condition::FirstUseEver)
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Hello mf  world!");
            ui.text("This...is...test eeny meeny minie moeshi bevy_mod_imgui!");
            ui.text("ayo");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position why it get bigger?: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });

    if state.demo_window_open {
        ui.show_demo_window(&mut state.demo_window_open);
    }
}
