use bevy::prelude::*;

pub mod background;
pub mod ball;
pub mod menu;
pub mod player;
pub mod powerup;

use background::BackgroundPlugin;
use ball::BallPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use powerup::PowerUpPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(MenuPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(PowerUpPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}