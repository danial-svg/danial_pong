use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}

fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // بارگذاری تصویر بک‌گراند
    let background_handle = asset_server.load("background.png");

    commands.spawn((
        Sprite {
            image: background_handle,
            // در صورت نیاز اندازه تصویر پس‌زمینه را تنظیم کنید
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        // محور Z برابر با 0.0 قرار داده شده تا پشت بقیه اشیاء (مثل Player با Z = 5.0) قرار بگیره
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}