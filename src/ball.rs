use bevy::app::AppExit;
use bevy::prelude::*;
use crate::player::{Player, PlayerId};

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub radius: f32,
    pub base_speed: f32,
    pub attached_to: Option<PlayerId>,
}

#[derive(Resource, Default)]
pub struct ScoreBoard {
    pub player_1: i32,
    pub player_2: i32,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreBoard>()
           .add_systems(Startup, setup_ball)
           .add_systems(Update, (move_ball, handle_collisions));
    }
}

fn setup_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ball_texture = asset_server.load("ball.png");

    commands.spawn((
        Sprite {
            image: ball_texture,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(-565.0, 0.0, 5.0),
        Ball {
            velocity: Vec2::new(350.0, 150.0),
            radius: 10.0,
            base_speed: 350.0,
            attached_to: Some(PlayerId::One),
        },
    ));
}

fn move_ball(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    player_query: Query<(&Transform, &Player), Without<Ball>>,
) {
    for (mut ball_transform, mut ball) in &mut ball_query {
        if let Some(owner_id) = ball.attached_to {
            for (player_transform, player) in &player_query {
                if player.id == owner_id {
                    let offset_x = if owner_id == PlayerId::One { 30.0 } else { -30.0 };
                    ball_transform.translation.x = player_transform.translation.x + offset_x;
                    ball_transform.translation.y = player_transform.translation.y;

                    let player_moved = match owner_id {
                        PlayerId::One => {
                            keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::KeyS)
                        }
                        PlayerId::Two => {
                            keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::ArrowDown)
                        }
                    };

                    if player_moved {
                        ball.attached_to = None;
                    }
                    break;
                }
            }
        } else {
            ball_transform.translation.x += ball.velocity.x * time.delta_secs();
            ball_transform.translation.y += ball.velocity.y * time.delta_secs();
        }
    }
}

fn handle_collisions(
    mut exit_events: MessageWriter<AppExit>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    player_query: Query<(&Transform, &Player), Without<Ball>>,
    mut scoreboard: ResMut<ScoreBoard>,
) {
    let window_height = 720.0;
    let window_width = 1280.0;
    let top_bound = window_height / 2.0;
    let bottom_bound = -window_height / 2.0;
    let left_bound = -window_width / 2.0;
    let right_bound = window_width / 2.0;

    let player_size = Vec2::new(35.0, 100.0);

    for (mut ball_transform, mut ball) in &mut ball_query {
        if ball.attached_to.is_some() {
            continue;
        }

        let ball_pos = ball_transform.translation.truncate();

        // ۱. برخورد با سقف و کف
        if ball_pos.y + ball.radius >= top_bound && ball.velocity.y > 0.0 {
            ball.velocity.y = -ball.velocity.y;
        }
        if ball_pos.y - ball.radius <= bottom_bound && ball.velocity.y < 0.0 {
            ball.velocity.y = -ball.velocity.y;
        }

        // ۲. برخورد با راکت‌ها
        for (player_transform, player) in &player_query {
            let player_pos = player_transform.translation.truncate();

            let is_colliding = (ball_pos.x - player_pos.x).abs() < (ball.radius + player_size.x / 2.0)
                && (ball_pos.y - player_pos.y).abs() < (ball.radius + player_size.y / 2.0);

            if is_colliding {
                let offset = (ball_pos.y - player_pos.y) / (player_size.y / 2.0);
                let bounce_angle = offset * (std::f32::consts::PI / 4.0);
                let speed = ball.velocity.length();

                match player.id {
                    PlayerId::One if ball.velocity.x < 0.0 => {
                        ball.velocity.x = speed * bounce_angle.cos();
                        ball.velocity.y = speed * bounce_angle.sin();
                    }
                    PlayerId::Two if ball.velocity.x > 0.0 => {
                        ball.velocity.x = -speed * bounce_angle.cos();
                        ball.velocity.y = speed * bounce_angle.sin();
                    }
                    _ => {}
                }
            }
        }

        // ۳. امتیازگیری و شرط خروج از بازی
        if ball_pos.x < left_bound {
            scoreboard.player_2 += 1;
            println!("Player 2 Scored! ({}-{})", scoreboard.player_1, scoreboard.player_2);

            if scoreboard.player_2 >= 3 {
                println!("🏆 PLAYER 2 WON THE GAME! Closing application...");
                exit_events.write(AppExit::Success);
                return;
            }

            reset_ball_to_player(&mut ball, PlayerId::Two, -1.0);

        } else if ball_pos.x > right_bound {
            scoreboard.player_1 += 1;
            println!("Player 1 Scored! ({}-{})", scoreboard.player_1, scoreboard.player_2);

            if scoreboard.player_1 >= 3 {
                println!("🏆 PLAYER 1 WON THE GAME! Closing application...");
                exit_events.write(AppExit::Success);
                return;
            }

            reset_ball_to_player(&mut ball, PlayerId::One, 1.0);
        }
    }
}

fn reset_ball_to_player(ball: &mut Ball, owner: PlayerId, direction: f32) {
    ball.base_speed += 50.0;
    ball.velocity = Vec2::new(ball.base_speed * direction, 100.0);
    ball.attached_to = Some(owner);
}