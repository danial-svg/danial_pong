use bevy::prelude::*;
use crate::menu::{GameMode, GameState};
use crate::ball::Ball;
use crate::powerup::ActiveEffects;

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PlayerId {
    One,
    Two,
}

#[derive(Component)]
pub struct Player {
    pub id: PlayerId,
    pub speed: f32,
    pub height: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Countdown), setup_player)
           .add_systems(OnEnter(GameState::MainMenu), cleanup_players)
           .add_systems(Update, move_player.run_if(in_state(GameState::InGame)));
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_players: Query<Entity, With<Player>>,
) {
    for entity in &existing_players {
        commands.entity(entity).despawn();
    }

    let idle_tex: Handle<Image> = asset_server.load("idle.png");

    // بازیکن ۱ (سمت چپ)
    commands.spawn((
        Sprite {
            image: idle_tex.clone(),
            custom_size: Some(Vec2::new(35.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(-600.0, 0.0, 5.0),
        Player {
            id: PlayerId::One,
            speed: 500.0,
            height: 100.0,
        },
        ActiveEffects::default(),
    ));

    // بازیکن ۲ (سمت راست - دوست یا AI)
    commands.spawn((
        Sprite {
            image: idle_tex,
            custom_size: Some(Vec2::new(35.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(600.0, 0.0, 5.0),
        Player {
            id: PlayerId::Two,
            speed: 500.0,
            height: 100.0,
        },
        ActiveEffects::default(),
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_mode: Res<GameMode>,
    ball_query: Query<&Transform, (With<Ball>, Without<Player>)>,
    mut query: Query<(&Player, &mut Transform, Option<&ActiveEffects>)>,
) {
    let ball_y = ball_query.single().map(|t| t.translation.y).unwrap_or(0.0);

    for (player, mut transform, effects) in &mut query {
        // ۱. بررسی افکت انجماد (اگر منجمد شده باشد کلا حرکت نمی‌کند)
        if let Some(eff) = effects {
            if eff.freeze_timer.is_some() {
                continue;
            }
        }

        let mut direction = 0.0;
        let is_reversed = effects.map_or(false, |e| e.reverse_timer.is_some());

        match player.id {
            PlayerId::One => {
                if keyboard_input.pressed(KeyCode::KeyW) { direction += 1.0; }
                if keyboard_input.pressed(KeyCode::KeyS) { direction -= 1.0; }
            }
            PlayerId::Two => {
                match *game_mode {
                    GameMode::TwoPlayers => {
                        if keyboard_input.pressed(KeyCode::ArrowUp) { direction += 1.0; }
                        if keyboard_input.pressed(KeyCode::ArrowDown) { direction -= 1.0; }
                    }
                    GameMode::AiEasy => {
                        let diff = ball_y - transform.translation.y;
                        if diff.abs() > 30.0 {
                            direction = diff.signum() * 0.55;
                        }
                    }
                    GameMode::AiHard => {
                        let diff = ball_y - transform.translation.y;
                        if diff.abs() > 10.0 {
                            direction = diff.signum() * 0.95;
                        }
                    }
                }
            }
        }

        // ۲. معکوس کردن جهت حرکت اگر افکت کنترل معکوس داشته باشد
        if is_reversed {
            direction *= -1.0;
        }

        // اعمال حرکت در محور Y و clamp کردن جهت نرفتن به بیرون صفحه
        transform.translation.y += direction * player.speed * time.delta_secs();
        transform.translation.y = transform.translation.y.clamp(-310.0, 310.0);
    }
}

fn cleanup_players(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}