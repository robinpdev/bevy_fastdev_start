use crate::common::{BOXHEIGHT, BOXWIDTH, CustomMaterial, ModuleWin};
use crate::module::{ModuleLayerCounter, spawn_module};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::module::{ModuleClass, SpawnModuleEvent, ResizeEvent};

// use bevy_simple_subsecond_system::prelude::*;

// Define your PlayerPlugin here, potentially combining systems from this module and sub-modules
pub struct BumpUiPlugin;

impl Plugin for BumpUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, ui_example_system);
    }
}


fn ui_example_system(
    mut ev_spawnmodule: EventWriter<SpawnModuleEvent>,
    mut ev_resize: EventWriter<ResizeEvent>,
    mut contexts: EguiContexts,
    query: Query<(Entity, &mut Transform, &mut ModuleWin, &mut Sprite)>,
    windows: Query<&mut Window>,
) -> Result {
    if let Ok(win) = windows.single() {
        // new window with our spawn button
        egui::SidePanel::left("Module Spawner").show(contexts.ctx_mut()?, |ui| {
            if ui.button("Spawn pong Module").clicked() {
                ev_spawnmodule.write(SpawnModuleEvent {
                    moduleclass: ModuleClass::Pong,
                });
            }
            if ui.button("Spawn noise Module").clicked() {
                ev_spawnmodule.write(SpawnModuleEvent {
                    moduleclass: ModuleClass::Noise,
                });
            }
        });

        for (entity, mut tf, mut mw, mut sprite) in query {
            let title = format!("{:?} module", mw.class);
            let window = egui::Window::new(title)
                .id(egui::Id::new(entity.index()))
                .pivot(egui::Align2::LEFT_TOP)
                .min_width(20.0)
                .min_height(20.0)
                .default_size([BOXWIDTH, BOXHEIGHT])
                .constrain(false)
                .title_bar(false)
                .frame(
                    egui::Frame::default()
                        .fill(egui::Color32::TRANSPARENT)
                        // .stroke(egui::Stroke::new(4.0, egui::Color32::BLACK)),
                )
                .show(contexts.ctx_mut()?, |ui| {
                    ui.allocate_space(ui.available_size());
                });

            // Get the current position after the window has been shown and potentially moved
            let response = window.and_then(|r| Some(r.response)).unwrap();
            let newsize = (
                (response.rect.size().x) as u32,
                (response.rect.size().y) as u32,
            );

            // set sprite custom size to window size if updated
            if (
                sprite.custom_size.unwrap().x as u32,
                sprite.custom_size.unwrap().y as u32,
            ) != newsize
            {
                sprite.custom_size = Some(Vec2 {
                    x: newsize.0 as f32,
                    y: newsize.1 as f32,
                });
                ev_resize.write(ResizeEvent {
                    target: entity,
                    width: newsize.0 as f32,
                    height: newsize.1 as f32,
                });
            }

            // set sprite position to window position
            tf.translation.x = response.rect.center().x - win.resolution.width() / 2.0;
            tf.translation.y = -response.rect.center().y + win.resolution.height() / 2.0;
        }
    }
    Ok(())
}
