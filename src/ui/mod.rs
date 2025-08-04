use crate::common::{BOXHEIGHT, BOXWIDTH, ModuleWin};

use bevy::prelude::*;
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui,
};

use bevy::{
    render::{
        render_resource::{Extent3d},
    },
};
use bevy_simple_subsecond_system::prelude::*;

// Define your PlayerPlugin here, potentially combining systems from this module and sub-modules
pub struct BumpUiPlugin;

impl Plugin for BumpUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, ui_example_system);
    }
}

#[hot]
fn ui_example_system(
    mut contexts: EguiContexts,
    query: Query<(&mut Transform, &mut ModuleWin)>,
    windows: Query<&mut Window>,
    mut assets: ResMut<Assets<Image>>,
) -> Result {
    let win = windows.single().unwrap();
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
    });

    for (mut tf, mut mw) in query {
        let window = egui::Window::new("module")
            .pivot(egui::Align2::LEFT_TOP)
            .min_width(40.0)
            .min_height(40.0)
            .default_size([BOXWIDTH, BOXHEIGHT])
            .constrain(false)
            .frame(egui::Frame::default().fill(egui::Color32::TRANSPARENT))
            .show(contexts.ctx_mut()?, |ui| {
                ui.label("yo");
                ui.allocate_space(ui.available_size());
            });

        // Get the current position after the window has been shown and potentially moved
        let response = window.and_then(|r| Some(r.response)).unwrap();

        tf.translation.x = response.rect.center().x - win.resolution.width() / 2.0;
        tf.translation.y = -response.rect.center().y + win.resolution.height() / 2.0;
        let newsize = ((response.rect.size().x) as u32, (response.rect.size().y) as u32);
        // tf.scale.x = response.rect.size().x / BOXWIDTH;
        // tf.scale.y = response.rect.size().y / BOXHEIGHT;


        
        
        if mw.resized && mw.size != newsize {
            // println!("FETCH");
            mw.size = newsize;
            let image = assets.get_mut(mw.image_h.id()).unwrap();
            // println!("RESIZE");
            let size = Extent3d {
                width: newsize.0,
                height: newsize.1,
                ..default()
            };
            image.resize(size);
        }else{
            
        }
        mw.resized = true;
    }

    Ok(())
}
