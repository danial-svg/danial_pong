use bevy::prelude::*;
use crate::player::{Player, PlayerId};
use crate::menu::GameState;
use crate::powerup::{ActiveEffects, ShieldEntity};

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

#[derive(Resource)]
pub struct CountdownTimer {
    pub timer: Timer,
    pub count: i32,
}

#[derive(Component)]
pub struct CountdownText;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct Player1ScoreText;

#[derive(Component)]
pub struct Player2ScoreText;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreBoard>()
           .add_systems(OnEnter(GameState::Countdown), (setup_countdown, setup_scoreboard_ui))
           .add_systems(Update, update_countdown.run_if(in_state(GameState::Countdown)))
           .add_systems(OnEnter(GameState::InGame), setup_ball)
           .add_systems(OnEnter(GameState::MainMenu), (cleanup_ball, cleanup_scoreboard_ui))
           .add_systems(
               Update,
               (move_ball, handle_collisions, update_scoreboard_ui).run_if(in_state(GameState::InGame)),
           );
    }
}

fn setup_scoreboard_ui(mut commands: Commands, existing_ui: Query<Entity, With<ScoreUI>>) {
    for entity in &existing_ui {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(80.0),
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(100.0)),
            ..default()
        },
        ScoreUI,
    )).with_children(|parent| {
        // امتیاز بازیکن ۱
        parent.spawn((
            Text::new("0"),
            TextFont { font_size: FontSize::Px(50.0), ..default() },
            TextColor(Color::WHITE),
            Player1ScoreText,
        ));

        // امتیاز بازیکن ۲
        parent.spawn((
            Text::new("0"),
            TextFont { font_size: FontSize::Px(50.0), ..default() },
            TextColor(Color::WHITE),
            Player2ScoreText,
        ));
    });
}

fn update_scoreboard_ui(
    scoreboard: Res<ScoreBoard>,
    mut p1_text: Query<&mut Text, (With<Player1ScoreText>, Without<Player2ScoreText>)>,
    mut p2_text: Query<&mut Text, (With<Player2ScoreText>, Without<Player1ScoreText>)>,
) {
    if scoreboard.is_changed() {
        if let Ok(mut text) = p1_text.single_mut() {
            **text = scoreboard.player_1.to_string();
        }
        if let Ok(mut text) = p2_text.single_mut() {
            **text = scoreboard.player_2.to_string();
        }
    }
}

fn setup_countdown(mut commands: Commands) {
    commands.insert_resource(CountdownTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        count: 3,
    });

    commands.spawn((
        Text::new("3"),
        TextFont { font_size: FontSize::Px(100.0), ..default() },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(48.0),
            top: Val::Percent(40.0),
            ..default()
        },
        CountdownText,
    ));
}

fn update_countdown(
    time: Res<Time>,
    mut timer_res: ResMut<CountdownTimer>,
    mut text_query: Query<&mut Text, With<CountdownText>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    text_entity: Query<Entity, With<CountdownText>>,
) {
    timer_res.timer.tick(time.delta());

    if timer_res.timer.just_finished() {
        timer_res.count -= 1;
        if timer_res.count > 0 {
            if let Ok(mut text) = text_query.single_mut() {
                **text = timer_res.count.to_string();
            }
        } else {
            if let Ok(e) = text_entity.single() {
                commands.entity(e).despawn();
            }
            next_state.set(GameState::InGame);
        }
    }
}

fn setup_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing_balls: Query<Entity, With<Ball>>,
    mut scoreboard: ResMut<ScoreBoard>,
) {
    for entity in &existing_balls {
        commands.entity(entity).despawn();
    }

    scoreboard.player_1 = 0;
    scoreboard.player_2 = 0;

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
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut player_query: Query<(&Transform, &mut Sprite, &mut Player, Option<&mut ActiveEffects>), Without<Ball>>,
    shields: Query<(Entity, &Transform, &ShieldEntity), (Without<Ball>, Without<Player>)>,
    mut scoreboard: ResMut<ScoreBoard>,
) {
    let window_height = 720.0;
    let window_width = 1280.0;
    let top_bound = window_height / 2.0;
    let bottom_bound = -window_height / 2.0;
    let left_bound = -window_width / 2.0;
    let right_bound = window_width / 2.0;

    for (mut ball_transform, mut ball) in &mut ball_query {
        if ball.attached_to.is_some() {
            continue;
        }

        let ball_pos = ball_transform.translation.truncate();

        if ball_pos.y + ball.radius >= top_bound && ball.velocity.y > 0.0 {
            ball.velocity.y = -ball.velocity.y;
        }
        if ball_pos.y - ball.radius <= bottom_bound && ball.velocity.y < 0.0 {
            ball.velocity.y = -ball.velocity.y;
        }

        for (shield_entity, shield_transform, shield) in &shields {
            let shield_x = shield_transform.translation.x;
            if (ball_pos.x - shield_x).abs() < (ball.radius + 5.0) {
                ball.velocity.x = -ball.velocity.x;
                commands.entity(shield_entity).despawn();

                for (_, _, player, effects) in player_query.iter_mut() {
                    if player.id == shield.owner {
                        if let Some(mut eff) = effects {
                            eff.shield_active = false;
                            eff.shield_timer = None;
                        }
                    }
                }
                break;
            }
        }

        for (player_transform, player_sprite, player, _) in &player_query {
            let player_pos = player_transform.translation.truncate();
            let p_size = player_sprite.custom_size.unwrap_or(Vec2::new(35.0, 100.0));

            let is_colliding = (ball_pos.x - player_pos.x).abs() < (ball.radius + p_size.x / 2.0)
                && (ball_pos.y - player_pos.y).abs() < (ball.radius + p_size.y / 2.0);

            if is_colliding {
                let offset = (ball_pos.y - player_pos.y) / (p_size.y / 2.0);
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

        if ball_pos.x < left_bound {
            scoreboard.player_2 += 1;
            shrink_opponent(&mut player_query, PlayerId::One);

            if scoreboard.player_2 >= 3 {
                next_state.set(GameState::MainMenu);
                return;
            }
            reset_ball_to_player(&mut ball, PlayerId::Two, -1.0);

        } else if ball_pos.x > right_bound {
            scoreboard.player_1 += 1;
            shrink_opponent(&mut player_query, PlayerId::Two);

            if scoreboard.player_1 >= 3 {
                next_state.set(GameState::MainMenu);
                return;
            }
            reset_ball_to_player(&mut ball, PlayerId::One, 1.0);
        }
    }
}

fn shrink_opponent(
    player_query: &mut Query<(&Transform, &mut Sprite, &mut Player, Option<&mut ActiveEffects>), Without<Ball>>,
    target_id: PlayerId,
) {
    for (_, mut sprite, mut player, _) in player_query.iter_mut() {
        if player.id == target_id {
            player.height = (player.height - 20.0).max(30.0);
            sprite.custom_size = Some(Vec2::new(35.0, player.height));
        }
    }
}

fn reset_ball_to_player(ball: &mut Ball, owner: PlayerId, direction: f32) {
    ball.base_speed += 30.0;
    ball.velocity = Vec2::new(ball.base_speed * direction, 100.0);
    ball.attached_to = Some(owner);
}

fn cleanup_ball(mut commands: Commands, query: Query<Entity, With<Ball>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn cleanup_scoreboard_ui(mut commands: Commands, query: Query<Entity, With<ScoreUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}