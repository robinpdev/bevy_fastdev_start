use crate::common::*;

use bevy::{image, prelude::*};
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui,
};
use bevy_simple_subsecond_system::prelude::*;
use bevy::input::common_conditions::*;


use bevy::{
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::RenderLayers,
    },
};

pub struct PongModulePlugin;

impl Plugin for PongModulePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Running), setup)
            .add_systems(
                Update,
                (rotator_system).run_if(in_state(AppState::Running)),
            )
            // .add_systems(Update, (
            //     handle_click
            //         .run_if(input_just_pressed(MouseButton::Left)),
            // ))
            ;
    }
}



#[hot]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut shadermaterials: ResMut<Assets<CustomMaterial>>,
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

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

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
    commands.spawn((Sprite::from_image(image_handle.clone()), ModuleWin{ image_h: image_handle.clone_weak(), resized: false, size: (BOXWIDTH as u32, BOXHEIGHT as u32) },));
}


fn handle_click(mut camwin: Query<&mut ModuleWin, With<Sprite>>, mut assets: ResMut<Assets<Image>>){
    let mwin = camwin.single().unwrap();

    let image = assets.get_mut(mwin.image_h.id()).unwrap();
    let size = Extent3d {
        width: 512,
        height: 712,
        ..default()
    };
    image.resize(size);
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