use bevy::prelude::*;

#[derive(Default, PartialEq, Eq, Clone, Copy, Hash, Debug, States)]
pub enum PlayerState {
    #[default]
    Winner,
    Loser,
}

// شناسه برای تفکیک بازیکن اول و دوم
#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerId {
    One,
    Two,
}

#[derive(Component)]
pub struct Player {
    pub id: PlayerId,
    pub point: i32,
    pub speed: f32,
}

#[derive(Resource)]
pub struct PlayerAnimationAssets {
    pub idle_texture: Handle<Image>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
           .add_systems(Update, (move_player, score_player));
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let idle_tex: Handle<Image> = asset_server.load("idle.png");

    commands.insert_resource(PlayerAnimationAssets {
        idle_texture: idle_tex.clone(),
    });

    // بازیکن ۱ (سمت چپ - کنترل با W و S)
    commands.spawn((
        Sprite {
            image: idle_tex.clone(),
            custom_size: Some(Vec2::new(35.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(-600.0, 0.0, 5.0),
        Player {
            id: PlayerId::One,
            point: 0,
            speed: 500.0,
        },
    ));

    // بازیکن ۲ (سمت راست - کنترل با ArrowUp و ArrowDown)
    commands.spawn((
        Sprite {
            image: idle_tex,
            custom_size: Some(Vec2::new(35.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(600.0, 0.0, 5.0),
        Player {
            id: PlayerId::Two,
            point: 0,
            speed: 500.0,
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut query {
        let mut direction = 0.0;

        // بررسی کلیدها بر اساس ID بازیکن
        match player.id {
            PlayerId::One => {
                if keyboard_input.pressed(KeyCode::KeyW) {
                    direction += 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    direction -= 1.0;
                }
            }
            PlayerId::Two => {
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    direction += 1.0;
                }
                if keyboard_input.pressed(KeyCode::ArrowDown) {
                    direction -= 1.0;
                }
            }
        }

        // اعمال حرکت در راستای محور Y
        transform.translation.y += direction * player.speed * time.delta_secs();

        // محدود کردن حرکت درون صفحه
        if transform.translation.y > 310.0 {
            transform.translation.y = 310.0;
        } else if transform.translation.y < -310.0 {
            transform.translation.y = -310.0;
        }
    }
}

fn score_player() {
    // منطق امتیاز
}