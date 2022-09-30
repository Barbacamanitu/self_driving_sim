//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use car::CarPlugin;
use road::RoadPlugin;

pub mod car;
pub mod math;
pub mod road;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CarPlugin)
        .add_plugin(RoadPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
}
