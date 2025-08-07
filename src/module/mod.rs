use crate::common::*;

use bevy::{prelude::*};
use bevy_simple_subsecond_system::prelude::*;


use bevy::{
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
};

#[derive(Resource)]
pub struct ModuleLayerCounter(pub u8);

pub struct PongModulePlugin;

impl Plugin for PongModulePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ModuleLayerCounter(1))
            .add_systems(OnEnter(AppState::Running), spawn_module)
            .add_systems(
                Update,
                (rotator_system).run_if(in_state(AppState::Running)),
            )
            .add_systems(Update, resize_images)
            // .add_systems(Update, (
            //     handle_click
            //         .run_if(input_just_pressed(MouseButton::Left)),
            // ))
            ;
    }
}

#[derive(Component)]
pub struct FirstPassEntity{
    spriteid: Entity,
}



#[hot]
pub fn spawn_module(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut shadermaterials: ResMut<Assets<CustomMaterial>>,
    mut layer_counter: ResMut<ModuleLayerCounter>,
) {
    println!("module setup!");
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

    //Sprite to display the rendered texture
    let mut sprite = Sprite::from_image(image_handle.clone());
    sprite.custom_size =  Some(Vec2{x: BOXWIDTH, y: BOXHEIGHT});
    let spriteid = commands.spawn((sprite, ModuleWin{ resized: false },)).id();

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(layer_counter.0 as usize);
    layer_counter.0 += 1;

    //first pass circle mesh
    //let circlemesh = meshes.add(Circle::new(200.0));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(RADIUS))),
        //MeshMaterial2d(colormaterials.add(Color::srgb(0.0, 1.0, 0.0))),
        MeshMaterial2d(shadermaterials.add(CustomMaterial {
            color: LinearRgba::RED,
        })),
        Transform::default(),
        HDirection::Right,
        VDirection::Up,
        FirstPassEntity{spriteid: spriteid},
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
}

#[hot]
fn resize_images(
    mut assets: ResMut<Assets<Image>>,
    wins: Query<(&Sprite, &mut ModuleWin)>,
){
    for (sprite, mut win) in wins{
        if win.resized{
            let image = assets.get_mut(&sprite.image).unwrap();
            
            let size = Extent3d {
                width: sprite.custom_size.unwrap().x as u32,
                height: sprite.custom_size.unwrap().y as u32,
                ..default()
            };
            image.resize(size);
            win.resized = false;
        }
    }
}

/// Rotates the inner cube (first pass)
#[hot]
fn rotator_system(
    mut query: Query<(&mut Transform, &mut VDirection, &mut HDirection, &FirstPassEntity)>,
    modules: Query<&Sprite, With<ModuleWin>>,
) {
    // for mut transform in &mut query {
    //     transform.rotate_x(1.5 * time.delta_secs());
    //     transform.rotate_z(0.4 * time.delta_secs());
    // }

    for (mut pos, mut vdir, mut hdir, fpe) in &mut query {
        let sprite = modules.get(fpe.spriteid).unwrap();
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