use crate::prelude::*;
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_event::<GoalEvent>()
            .add_systems(OnEnter(GameState::Pong), setup_pong)
            .add_systems(Update, player_movement)
            .add_systems(Update, ball_movement)
            .add_systems(Update, opponent_movement)
            .add_systems(Update, score_goal.after(ball_movement))
            .add_systems(
                Update,
                update_score
                    .after(score_goal)
                    .run_if(in_state(GameState::Pong)),
            );
    }
}

const PADDLE_HEIGHT: f32 = 50.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_SPEED: f32 = 100.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Opponent;

#[derive(Component)]
struct Ball {
    direction: Vec2,
}

#[derive(Resource, Default)]
struct Score {
    player: i32,
    opponent: i32,
}

#[derive(Event)]
struct GoalEvent {
    is_player_goal: bool,
}

fn setup_pong(mut commands: Commands) {
    // Player
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 10.0),
                translation: Vec3::new(-((WINDOW_HEIGHT - PADDLE_WIDTH) / 2.0), 0.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
    // Opponent
    commands.spawn((
        Opponent,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 10.0),
                translation: Vec3::new((WINDOW_WIDTH - PADDLE_WIDTH) / 2.0, 0.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
    // Ball
    commands.spawn((
        Ball {
            direction: Vec2::new(3.0, 2.0).normalize(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_WIDTH, 10.0),
                ..default()
            },
            ..default()
        },
    ));
    // Score
    commands.spawn(
        TextBundle::from_sections([TextSection::from_style(TextStyle::default())]).with_style(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..default()
                },
                ..default()
            },
        ),
    );
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
) {
    let Some(mut player_transform) = transform_query.iter_mut().next() else {
        return;
    };

    let top_boundary = (WINDOW_HEIGHT - PADDLE_HEIGHT) / 2.0;
    let bottom_boundary = top_boundary * -1.0;
    if keyboard_input.pressed(KeyCode::Up) {
        player_transform.translation.y = (player_transform.translation.y
            + time.delta_seconds() * PADDLE_SPEED)
            .clamp(bottom_boundary, top_boundary);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player_transform.translation.y = (player_transform.translation.y
            - time.delta_seconds() * PADDLE_SPEED)
            .clamp(bottom_boundary, top_boundary);
    }
}

fn opponent_movement(
    time: Res<Time>,
    mut opponent_transform_query: Query<&mut Transform, (Without<Ball>, With<Opponent>)>,
    ball_query: Query<&Transform, With<Ball>>,
) {
    let Some(mut opponent_transform) = opponent_transform_query.iter_mut().next() else {
        return;
    };
    let Some(ball_transform) = ball_query.iter().next() else {
        return;
    };

    let distance_to_ball = opponent_transform.translation.y - ball_transform.translation.y;
    let top_boundary = (WINDOW_HEIGHT - PADDLE_HEIGHT) / 2.0;
    let bottom_boundary = top_boundary * -1.0;

    if distance_to_ball > 0.0 {
        opponent_transform.translation.y = (opponent_transform.translation.y
            - time.delta_seconds() * PADDLE_SPEED)
            .max(ball_transform.translation.y);
    } else if distance_to_ball < 0.0 {
        opponent_transform.translation.y = (opponent_transform.translation.y
            + time.delta_seconds() * PADDLE_SPEED)
            .min(ball_transform.translation.y);
    }

    opponent_transform.translation.y = opponent_transform
        .translation
        .y
        .clamp(bottom_boundary, top_boundary);
}

fn ball_movement(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    player_transform_query: Query<&Transform, (Without<Ball>, With<Player>)>,
    opponent_transform_query: Query<&Transform, (Without<Ball>, With<Opponent>)>,
    mut goal_event_writer: EventWriter<GoalEvent>,
) {
    let Some((mut ball_transform, mut ball)) = ball_query.iter_mut().next() else {
        return;
    };

    let top_boundary = (WINDOW_WIDTH - PADDLE_WIDTH) / 2.0;
    let bottom_boundary = top_boundary * -1.0;

    ball_transform.translation.x += time.delta_seconds() * PADDLE_SPEED * ball.direction.x;
    ball_transform.translation.y += time.delta_seconds() * PADDLE_SPEED * ball.direction.y;

    let x = ball_transform.translation.x;
    let y = ball_transform.translation.y;

    if x >= top_boundary || x <= bottom_boundary {
        goal_event_writer.send(GoalEvent {
            is_player_goal: x > 0.0,
        });
        return;
    }

    if y >= top_boundary || y <= bottom_boundary {
        ball_transform.translation.y = ball_transform
            .translation
            .y
            .clamp(bottom_boundary + 1.0, top_boundary - 1.0);
        ball.direction.y *= -1.0;
    }

    if let Some(player_transform) = player_transform_query.iter().next() {
        if ball.direction.x < 0.0
            && collide(
                ball_transform.translation,
                ball_transform.scale.truncate(),
                player_transform.translation,
                player_transform.scale.truncate(),
            )
            .is_some()
        {
            ball.direction.x *= -1.0;
        }
    }

    if let Some(opponent_transform) = opponent_transform_query.iter().next() {
        if ball.direction.x > 0.0
            && collide(
                ball_transform.translation,
                ball_transform.scale.truncate(),
                opponent_transform.translation,
                opponent_transform.scale.truncate(),
            )
            .is_some()
        {
            ball.direction.x *= -1.0;
        }
    }
}

fn score_goal(
    mut goal_event_reader: EventReader<GoalEvent>,
    mut score: ResMut<Score>,
    mut ball_query: Query<(&mut Transform, &mut Ball), (Without<Player>, Without<Opponent>)>,
    mut player_transform_query: Query<
        (&mut Transform, &Player),
        (Without<Ball>, Without<Opponent>),
    >,
    mut opponent_transform_query: Query<
        (&mut Transform, &Opponent),
        (Without<Ball>, Without<Player>),
    >,
) {
    if let Some(goal_event) = goal_event_reader.read().next() {
        if goal_event.is_player_goal {
            score.player += 1;
        } else {
            score.opponent += 1;
        }

        let Some((mut ball_transform, mut ball)) = ball_query.iter_mut().next() else {
            return;
        };

        ball.direction.x *= -1.0;
        ball_transform.translation = Vec3::splat(0.0);

        let Some((mut opponent_transform, _)) = opponent_transform_query.iter_mut().next() else {
            return;
        };
        let Some((mut player_transform, _)) = player_transform_query.iter_mut().next() else {
            return;
        };

        opponent_transform.translation.y = 0.0;
        player_transform.translation.y = 0.0;
    }
}

fn update_score(score: Res<Score>, mut text_query: Query<&mut Text>) {
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("{} | {}", score.player, score.opponent);
}
