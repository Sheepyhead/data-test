use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Area, RichText},
    EguiContext,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

use crate::controls::PlayerDestinationIndicator;

pub struct Debug;

impl Plugin for Debug {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WorldInspectorParams {
            enabled: false,
            ..default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(toggle_inspector)
        .add_system(debug_overlay);
    }
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled;
    }
}

fn debug_overlay(
    mut ctx: ResMut<EguiContext>,
    window_params: Res<WorldInspectorParams>,
    indicators: Query<&GlobalTransform, With<PlayerDestinationIndicator>>,
) {
    if !window_params.enabled {
        return;
    }

    Area::new("DebugOverlay")
        .anchor(Align2::RIGHT_TOP, (0., 0.))
        .movable(false)
        .interactable(false)
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.heading(text(format!(
                    "Current player destination: {:?}",
                    indicators
                        .get_single()
                        .map(GlobalTransform::translation)
                        .ok()
                )))
            })
        });
}

fn text(text: String) -> RichText {
    RichText::new(text)
}
