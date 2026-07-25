use bevy::prelude::*;
use rand::Rng;
use crate::ball::Ball;
use crate::player::{Player, PlayerId};
use crate::menu::GameState;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpType {
    BigPaddle,      // راکت بزرگ (خود فرد)
    SmallPaddle,    // راکت کوچک (حریف)
    FastBall,       // توپ سریع
    SlowBall,       // توپ کند
    FreezeOpponent, // حریف منجمد
    Shield,         // سپر دفاعی
    ReverseControls,// کنترل معکوس حریف
    MultiBall,      // چند توپی
}

impl PowerUpType {
    pub fn color(&self) -> Color {
        match self {
            PowerUpType::BigPaddle => Color::srgb(0.0, 1.0, 0.0),       // سبز
            PowerUpType::SmallPaddle => Color::srgb(1.0, 0.0, 0.0),     // قرمز
            PowerUpType::FastBall => Color::srgb(1.0, 0.5, 0.0),        // نارنجی
            PowerUpType::SlowBall => Color::srgb(0.0, 0.8, 1.0),        // آبی روشن
            PowerUpType::FreezeOpponent => Color::srgb(0.0, 0.2, 1.0),  // آبی تیره
            PowerUpType::Shield => Color::srgb(0.9, 0.9, 0.0),          // زرد
            PowerUpType::ReverseControls => Color::srgb(0.8, 0.0, 0.8),// بنفش
            PowerUpType::MultiBall => Color::srgb(1.0, 1.0, 1.0),       // سفید
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..8) {
            0 => PowerUpType::BigPaddle,
            1 => PowerUpType::SmallPaddle,
            2 => PowerUpType::FastBall,
            3 => PowerUpType::SlowBall,
            4 => PowerUpType::FreezeOpponent,
            5 => PowerUpType::Shield,
            6 => PowerUpType::ReverseControls,
            _ => PowerUpType::MultiBall,
        }
    }
}

#[derive(Component)]
pub struct PowerUpItem {
    pub power_type: PowerUpType,
    pub radius: f32,
}

#[derive(Resource)]
pub struct PowerUpSpawnTimer(pub Timer);

#[derive(Component, Default)]
pub struct ActiveEffects {
    pub big_paddle_timer: Option<Timer>,
    pub small_paddle_timer: Option<Timer>,
    pub freeze_timer: Option<Timer>,
    pub reverse_timer: Option<Timer>,
    pub shield_active: bool,
    pub shield_timer: Option<Timer>,
}

#[derive(Component)]
pub struct ShieldEntity {
    pub owner: PlayerId,
}

#[derive(Component)]
pub struct ExtraBall;

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PowerUpSpawnTimer(Timer::from_seconds(8.0, TimerMode::Repeating)))
           .add_systems(OnEnter(GameState::InGame), setup_effects)
           .add_systems(OnEnter(GameState::MainMenu), cleanup_powerups)
           .add_systems(
               Update,
               (
                   spawn_powerup,
                   check_ball_powerup_collision,
                   update_active_effects,
               ).run_if(in_state(GameState::InGame)),
           );
    }
}

fn setup_effects(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for entity in &players {
        commands.entity(entity).insert(ActiveEffects::default());
    }
}

fn spawn_powerup(
    time: Res<Time>,
    mut timer: ResMut<PowerUpSpawnTimer>,
    existing_items: Query<Entity, With<PowerUpItem>>,
    mut commands: Commands,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        for entity in &existing_items {
            commands.entity(entity).despawn();
        }

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-400.0..400.0);
        let y = rng.gen_range(-250.0..250.0);
        let p_type = PowerUpType::random();

        commands.spawn((
            Sprite {
                color: p_type.color(),
                // سایز قبلی 30.0 بود که ضرب در 2.5 می‌شود 75.0
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 4.0),
            PowerUpItem {
                power_type: p_type,
                // شعاع شعاع برخورد هم 2.5 برابر می‌شود (از 15.0 به 37.5)
                radius: 30.0,
            },
        ));
    }
}

fn check_ball_powerup_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    balls: Query<(&Transform, &Ball)>,
    powerups: Query<(Entity, &Transform, &PowerUpItem)>,
    mut players: Query<(&Player, &mut ActiveEffects, &mut Sprite)>,
) {
    for (ball_transform, ball) in &balls {
        let ball_pos = ball_transform.translation.truncate();

        for (p_entity, p_transform, p_item) in &powerups {
            let p_pos = p_transform.translation.truncate();

            if ball_pos.distance(p_pos) < (ball.radius + p_item.radius) {
                let hitter_id = if ball.velocity.x > 0.0 { PlayerId::One } else { PlayerId::Two };
                let opponent_id = if hitter_id == PlayerId::One { PlayerId::Two } else { PlayerId::One };

                apply_powerup(
                    &mut commands,
                    &asset_server,
                    p_item.power_type,
                    hitter_id,
                    opponent_id,
                    &mut players,
                    ball_transform.translation,
                );

                commands.entity(p_entity).despawn();
            }
        }
    }
}

fn apply_powerup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    p_type: PowerUpType,
    hitter: PlayerId,
    opponent: PlayerId,
    players: &mut Query<(&Player, &mut ActiveEffects, &mut Sprite)>,
    ball_pos: Vec3,
) {
    match p_type {
        PowerUpType::BigPaddle => {
            for (player, mut effects, mut sprite) in players.iter_mut() {
                if player.id == hitter {
                    effects.big_paddle_timer = Some(Timer::from_seconds(5.0, TimerMode::Once));
                    sprite.custom_size = Some(Vec2::new(35.0, player.height * 2.0));
                }
            }
        }
        PowerUpType::SmallPaddle => {
            for (player, mut effects, mut sprite) in players.iter_mut() {
                if player.id == opponent {
                    effects.small_paddle_timer = Some(Timer::from_seconds(5.0, TimerMode::Once));
                    sprite.custom_size = Some(Vec2::new(35.0, player.height * 0.5));
                }
            }
        }
        PowerUpType::FreezeOpponent => {
            for (player, mut effects, _) in players.iter_mut() {
                if player.id == opponent {
                    effects.freeze_timer = Some(Timer::from_seconds(3.0, TimerMode::Once));
                }
            }
        }
        PowerUpType::ReverseControls => {
            for (player, mut effects, _) in players.iter_mut() {
                if player.id == opponent {
                    effects.reverse_timer = Some(Timer::from_seconds(4.0, TimerMode::Once));
                }
            }
        }
        PowerUpType::Shield => {
            for (player, mut effects, _) in players.iter_mut() {
                if player.id == hitter {
                    effects.shield_active = true;
                    effects.shield_timer = Some(Timer::from_seconds(10.0, TimerMode::Once));

                    let shield_x = if hitter == PlayerId::One { -620.0 } else { 620.0 };
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.9, 0.9, 0.0),
                            custom_size: Some(Vec2::new(10.0, 720.0)),
                            ..default()
                        },
                        Transform::from_xyz(shield_x, 0.0, 4.0),
                        ShieldEntity { owner: hitter },
                    ));
                }
            }
        }
        PowerUpType::MultiBall => {
            // بارگذاری تصویر ball.png برای توپ دوم
            let ball_texture = asset_server.load("ball.png");

            commands.spawn((
                Sprite {
                    image: ball_texture,
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                Transform::from_translation(ball_pos),
                Ball {
                    velocity: Vec2::new(300.0, -200.0),
                    radius: 10.0,
                    base_speed: 300.0,
                    attached_to: None,
                },
                ExtraBall,
            ));
        }
        _ => {}
    }
}

fn update_active_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut players: Query<(&Player, &mut ActiveEffects, &mut Sprite)>,
    shields: Query<(Entity, &ShieldEntity)>,
) {
    for (player, mut effects, mut sprite) in &mut players {
        if let Some(ref mut timer) = effects.big_paddle_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.custom_size = Some(Vec2::new(35.0, player.height));
                effects.big_paddle_timer = None;
            }
        }

        if let Some(ref mut timer) = effects.small_paddle_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.custom_size = Some(Vec2::new(35.0, player.height));
                effects.small_paddle_timer = None;
            }
        }

        if let Some(ref mut timer) = effects.freeze_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                effects.freeze_timer = None;
            }
        }

        if let Some(ref mut timer) = effects.reverse_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                effects.reverse_timer = None;
            }
        }

        if let Some(ref mut timer) = effects.shield_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                effects.shield_active = false;
                effects.shield_timer = None;
                for (s_entity, shield) in &shields {
                    if shield.owner == player.id {
                        commands.entity(s_entity).despawn();
                    }
                }
            }
        }
    }
}

fn cleanup_powerups(
    mut commands: Commands,
    items: Query<Entity, With<PowerUpItem>>,
    shields: Query<Entity, With<ShieldEntity>>,
) {
    for entity in &items {
        commands.entity(entity).despawn();
    }
    for entity in &shields {
        commands.entity(entity).despawn();
    }
}