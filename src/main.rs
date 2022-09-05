#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::multiple_crate_versions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::must_use_candidate,
    clippy::enum_glob_use
)]

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_tweening::TweeningPlugin;
use controls::Controls;
use debug::Debug;
use gltf::GltfPlugin;
use movement::Movement;
use physics::Physics;
use player::PlayerPlugin;
use tiles::Tiles;

mod camera;
mod common;
mod controls;
mod custom_meshes;
mod debug;
mod gltf;
mod movement;
mod physics;
mod player;
mod tiles;

pub const CLEAR: Color = Color::BLACK;
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            present_mode: PresentMode::Fifo,
            resizable: false,
            mode: WindowMode::Windowed,
            position: WindowPosition::Centered(MonitorSelection::Number(0)),
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.7,
        })
        // External plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        // Internal plugins
        .add_plugin(camera::CameraPlugin)
        .add_plugin(Controls)
        .add_plugin(Debug)
        .add_plugin(Movement)
        .add_plugin(Physics)
        .add_plugin(PlayerPlugin)
        .add_plugin(Tiles)
        .add_plugin(GltfPlugin)
        .run();
}
