use crate::common::*;

use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

//import noisemodule
mod noise;
mod pong;

use bevy::render::{
    render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::RenderLayers;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModuleClass {
    Pong,
    Noise,
}

trait HasModuleClass {
    fn get_module_class(&self) -> ModuleClass;
}

#[derive(Message)]
pub struct SpawnModuleEvent {
    pub moduleclass: ModuleClass,
}

#[derive(Message)]
pub struct SpawnModuleInternalEvent {
    pub moduleclass: ModuleClass,
    pub layer: RenderLayers,
    pub moduleID: Entity,
}

impl HasModuleClass for SpawnModuleEvent {
    fn get_module_class(&self) -> ModuleClass {
        self.moduleclass
    }
}

impl HasModuleClass for SpawnModuleInternalEvent {
    fn get_module_class(&self) -> ModuleClass {
        self.moduleclass
    }
}


#[derive(Message)]
pub struct ResizeEvent {
    pub target: Entity,
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct ModuleLayerCounter(pub u8);

pub struct ModulePlugin;

pub fn run_if_module<T>(class: ModuleClass) -> impl Fn(MessageReader<T>) -> bool where T : Message + HasModuleClass {
    move |mut evspawn| {
        for ev in evspawn.read() {
            if ev.get_module_class() == class {
                return true;
            }
        }
        false
    }
}

impl Plugin for ModulePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ModuleLayerCounter(1))
            .add_event::<SpawnModuleEvent>()
            .add_event::<SpawnModuleInternalEvent>()
            .add_event::<ResizeEvent>()
            .add_systems(PreUpdate, spawn_module.run_if(on_event::<SpawnModuleEvent>))
            .add_systems(Update, resize_images.run_if(on_event::<ResizeEvent>))
            .add_plugins(noise::NoiseModule)
            .add_plugins(pong::PongModule)
            // .add_systems(Update, (
            //     handle_click
            //         .run_if(input_just_pressed(MouseButton::Left)),
            // ))
            ;
    }
}

#[derive(Component)]
pub struct FirstPassEntity {
    ModuleId: Entity,
}

#[hot]
pub fn spawn_module(
    mut ev_spawn: MessageReader<SpawnModuleEvent>,
    mut ev_spawnmodule: EventWriter<SpawnModuleInternalEvent>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut layer_counter: ResMut<ModuleLayerCounter>,
) {
    println!("module setup!");
    for ev in ev_spawn.read() {
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
            &[0, 0, 0, 255],
            TextureFormat::Bgra8UnormSrgb,
            RenderAssetUsages::default(),
        );

        // You need to set these texture usage flags in order to use the image as a render target
        image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT;

        let image_handle = images.add(image);

        //Sprite to display the rendered texture
        let mut sprite = Sprite::from_image(image_handle.clone());
        sprite.custom_size = Some(Vec2 {
            x: BOXWIDTH,
            y: BOXHEIGHT,
        });
        let spriteid = commands
            .spawn((
                sprite,
                ModuleWin { class: ev.moduleclass },
                Transform::from_translation(Vec3::new(0.0, 0.0, layer_counter.0 as f32 * 0.01)),
            ))
            .id();

        // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
        let first_pass_layer = RenderLayers::layer(layer_counter.0 as usize);
        layer_counter.0 += 1;

        //first pass camera
        commands.spawn((
            Camera2d::default(),
            Camera {
                target: image_handle.clone().into(),
                clear_color: Color::WHITE.into(),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)).looking_at(Vec3::ZERO, Vec3::Y),
            first_pass_layer.clone(),
        ));

        ev_spawnmodule.write(SpawnModuleInternalEvent {
            moduleclass: ev.moduleclass,
            layer: first_pass_layer.clone(),
            moduleID: spriteid,
        });
    }
}

#[hot]
fn resize_images(
    mut assets: ResMut<Assets<Image>>,
    wins: Query<(&Sprite, &mut ModuleWin)>,
    mut ev_resize: MessageReader<ResizeEvent>,
) {
    for ev in ev_resize.read() {
        if let Ok((sprite, mut win)) = wins.get(ev.target) {
            let image = assets.get_mut(&sprite.image).unwrap();

            let size = Extent3d {
                width: ev.width as u32,
                height: ev.height as u32,
                ..default()
            };
            image.resize(size);
        }
    }
}
