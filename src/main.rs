use asset_manager::AssetManagerPlugin;
use bevy::prelude::*;

use camera::CameraPlugin;
// use debug::DebugPlugin;
use player::{MovementPlugin, PlayerPlugin};
use schedule::HandleGameState;

mod player;
// mod debug;
mod schedule;
mod asset_manager;
mod camera;

fn main() {
    App::new()

        // set resources premade for bevy environment
        .insert_resource(ClearColor(Color::srgb(242.0 / 255.0, 209.0 / 255.0, 125.0 / 255.0)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 3000.0,
        })
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AssetManagerPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(HandleGameState)
        .add_plugins(PlayerPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
