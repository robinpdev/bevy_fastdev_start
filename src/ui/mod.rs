use crate::common::{
    BOXHEIGHT, BOXWIDTH, ModuleWin
};

use bevy::prelude::*;
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui,
};
use bevy_simple_subsecond_system::prelude::*;


// Define your PlayerPlugin here, potentially combining systems from this module and sub-modules
pub struct BumpUiPlugin;

impl Plugin for BumpUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, ui_example_system)
            ;
    }
}

#[hot]
fn ui_example_system(
    mut contexts: EguiContexts,
    query: Query<&mut Transform, With<ModuleWin>>,
    windows: Query<&mut Window>,
    ) -> Result {
    let mw = windows.single().unwrap();
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
    });

    for(mut tf) in query{
        let window = egui::Window::new("module")
        .pivot(egui::Align2::LEFT_TOP)
        .min_width(BOXWIDTH)
        .min_height(BOXHEIGHT)
        .default_size([BOXWIDTH, BOXHEIGHT]) 
        .constrain(false)
        .frame(egui::Frame::default().fill(egui::Color32::TRANSPARENT))
        .show(contexts.ctx_mut()?, |ui| {
            ui.label("yo");
            ui.allocate_space(ui.available_size());
        });

        // Get the current position after the window has been shown and potentially moved
        let response = window.and_then(|r| Some(r.response)).unwrap();
        
        tf.translation.x = response.rect.min.x - BOXWIDTH / 9.4;
        tf.translation.y = -response.rect.min.y - BOXHEIGHT / 18.4;
    }

    Ok(())
}