use bevy::prelude::*;

pub mod background;
pub mod ball;
pub mod player;

use background::BackgroundPlugin;
use ball::BallPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(BackgroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin) 
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}