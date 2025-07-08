mod common;
mod ui;

use common::*;

// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy::diagnostic::LogDiagnosticsPlugin;

use bevy_simple_subsecond_system::prelude::*;
use iyes_perf_ui::prelude::*;

use bevy::{
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
    window::{PresentMode, WindowTheme},
};

use bevy::ecs::schedule::ExecutorKind;
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgba(0.2, 0.2, 0.2, 1.0)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "I am the window!".into(),
                        name: Some("bevy.app".into()),
                        resolution: (500., 300.).into(),
                        // present_mode: PresentMode::AutoNoVsync,
                        // Tells Wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        // window_theme: Some(WindowTheme::Dark),
                        // enabled_buttons: bevy::window::EnabledButtons {
                        //     maximize: false,
                        //     ..Default::default()
                        // },
                        // This will spawn an invisible window
                        // The window will be made visible in the make_visible() system after 3 frames.
                        // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }),
                // .set(AssetPlugin {
                //     watch_for_changes_override: Some(true),
                //     ..Default::default()
                // }),
            // LogDiagnosticsPlugin::default(),
            bevy_framepace::FramepacePlugin,
        ))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PerfUiPlugin)
        .edit_schedule(Update, |schedule| {
            schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        })
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Restarting), restart)
        .add_systems(OnEnter(AppState::Running), setup)
        .add_systems(OnExit(AppState::Running), teardown)
        // .add_plugins(bevy_mod_imgui::ImguiPlugin {
        //     ini_filename: Some("hello-world.ini".into()),
        //     font_oversample_h: 2,
        //     font_oversample_v: 2,
        //     ..default()
        // })
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(
            Update,
            (greet, rotator_system).run_if(in_state(AppState::Running)),
        )
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_systems(Startup, restart)
        .add_systems(PreUpdate, trigger_restart)
        .add_systems(PreStartup, spawn_immortals)
        .add_plugins(ui::BumpUiPlugin);

    app.run();
}




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
        ),
    >,
) {
    // if you want to see what components that entities about to be despawned have

    // for entity in query.iter() {
    //     if let Some(component_names) = get_component_names(world, entity) {
    //         println!("Component names for entity {:?}: {:?}", entity, component_names);
    //     } else {
    //         // This branch is now reached if the entity doesn't exist.
    //         println!("Entity {:?} does not exist.", entity);
    //     }
    // }

    println!("teardown!");
    for entity in query.iter() {
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



/// Runs each time the scene is (re)started
/// Sets up a circle that gets rendered to a texture and then shown on the main context
#[hot]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colormaterials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut shadermaterials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("setup!");

    // main camera
    commands.spawn(Camera2d);

    // rendered texture
    let size = Extent3d {
        width: BOXWIDTH as u32,
        height: BOXHEIGHT as u32,
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
    //let circlemesh = meshes.add(Circle::new(200.0));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(RADIUS))),
        //MeshMaterial2d(colormaterials.add(Color::srgb(0.0, 1.0, 0.0))),
        MeshMaterial2d(shadermaterials.add(CustomMaterial {
            color: LinearRgba::BLUE,
        })),
        Transform::default(),
        HDirection::Right,
        VDirection::Up,
        FirstPassEntity,
        first_pass_layer.clone(),
    ));

    //first pass camera
    commands.spawn((
        Camera2d::default(),
        Camera {
            target: image_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)).looking_at(Vec3::ZERO, Vec3::Y),
        first_pass_layer,
    ));

    //Sprite to display the rendered texture
    commands.spawn((Sprite::from_image(image_handle.clone()), ModuleWin));

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
}



/// Rotates the inner cube (first pass)
#[hot]
fn rotator_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut VDirection, &mut HDirection)>,
) {
    // for mut transform in &mut query {
    //     transform.rotate_x(1.5 * time.delta_secs());
    //     transform.rotate_z(0.4 * time.delta_secs());
    // }

    for (mut pos, mut vdir, mut hdir) in &mut query {
        match *vdir {
            VDirection::Up => pos.translation.y += SPEED,
            VDirection::Down => pos.translation.y -= SPEED,
        }

        match *hdir {
            HDirection::Left => pos.translation.x -= SPEED,
            HDirection::Right => pos.translation.x += SPEED,
        }

        if pos.translation.x + RADIUS > BOXWIDTH / 2 as f32 {
            *hdir = HDirection::Left
        } else if pos.translation.x - RADIUS < -BOXWIDTH / 2 as f32 {
            *hdir = HDirection::Right
        }

        if pos.translation.y + RADIUS > BOXHEIGHT / 2 as f32 {
            *vdir = VDirection::Down
        } else if pos.translation.y - RADIUS < -BOXHEIGHT / 2 as f32 {
            *vdir = VDirection::Up
        }
    }
}



// fn imgui_example_ui(mut context: NonSendMut<ImguiContext>, mut state: ResMut<ImguiState>) {
//     let ui = context.ui();
//     let window = ui.window("Hello mf world");
//     window
//         .size([300.0, 100.0], imgui::Condition::FirstUseEver)
//         .position([0.0, 0.0], imgui::Condition::FirstUseEver)
//         .build(|| {
//             ui.text("Hello mf  world!");
//             ui.text("This...is...test eeny meeny minie moeshi bevy_mod_imgui!");
//             ui.text("ayo");
//             ui.separator();
//             let mouse_pos = ui.io().mouse_pos;
//             ui.text(format!(
//                 "Mouse Position why it get bigger?: ({:.1},{:.1})",
//                 mouse_pos[0], mouse_pos[1]
//             ));
//         });

//     if state.demo_window_open {
//         ui.show_demo_window(&mut state.demo_window_open);
//     }
// }

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
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

