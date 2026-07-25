use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    AiDifficultyMenu,
    Countdown,
    InGame,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameMode {
    #[default]
    TwoPlayers,
    AiEasy,
    AiHard,
}

#[derive(Component)]
pub struct MenuUI;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
           .init_resource::<GameMode>()
           .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
           .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
           .add_systems(OnEnter(GameState::AiDifficultyMenu), setup_ai_menu)
           .add_systems(OnExit(GameState::AiDifficultyMenu), cleanup_menu)
           .add_systems(
               Update,
               handle_menu_buttons.run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::AiDifficultyMenu))),
           );
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(15.0),
            ..default()
        },
        MenuUI,
    )).with_children(|parent| {
        // ۱. دکمه بازی ۲ نفره
        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
            Name::new("BtnPvp"),
        )).with_children(|b| {
            b.spawn((Text::new("2 Player Mode"), TextFont { font_size: FontSize::Px(25.0), ..default() }));
        });

        // ۲. دکمه بازی با هوش مصنوعی
        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.4, 0.8)),
            Name::new("BtnAi"),
        )).with_children(|b| {
            b.spawn((Text::new("VS AI Mode"), TextFont { font_size: FontSize::Px(25.0), ..default() }));
        });

        // ۳. دکمه خروج از بازی
        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
            Name::new("BtnQuit"),
        )).with_children(|b| {
            b.spawn((Text::new("Exit Game"), TextFont { font_size: FontSize::Px(25.0), ..default() }));
        });
    });
}

fn setup_ai_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(15.0),
            ..default()
        },
        MenuUI,
    )).with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.3, 0.7, 0.3)),
            Name::new("BtnEasy"),
        )).with_children(|b| {
            b.spawn((Text::new("Easy AI"), TextFont { font_size: FontSize::Px(25.0), ..default() }));
        });

        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.3, 0.3)),
            Name::new("BtnHard"),
        )).with_children(|b| {
            b.spawn((Text::new("Hard AI"), TextFont { font_size: FontSize::Px(25.0), ..default() }));
        });
    });
}

fn handle_menu_buttons(
    mut interaction_query: Query<(&Interaction, &Name), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_mode: ResMut<GameMode>,
    mut exit_events: MessageWriter<AppExit>,
) {
    for (interaction, name) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match name.as_str() {
                "BtnPvp" => {
                    *game_mode = GameMode::TwoPlayers;
                    next_state.set(GameState::Countdown);
                }
                "BtnAi" => {
                    next_state.set(GameState::AiDifficultyMenu);
                }
                "BtnEasy" => {
                    *game_mode = GameMode::AiEasy;
                    next_state.set(GameState::Countdown);
                }
                "BtnHard" => {
                    *game_mode = GameMode::AiHard;
                    next_state.set(GameState::Countdown);
                }
                "BtnQuit" => {
                    exit_events.write(AppExit::Success);
                }
                _ => {}
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}