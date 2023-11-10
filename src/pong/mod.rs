use crate::prelude::*;
use bevy::prelude::*;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Pong), setup_pong)
            .add_systems(Update, player_movement)
            .add_systems(Update, ball_movement);
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

fn ball_movement(time: Res<Time>, mut ball_query: Query<(&mut Transform, &mut Ball)>) {
    let Some((mut ball_transform, mut ball)) = ball_query.iter_mut().next() else {
        return;
    };

    let top_boundary = ((WINDOW_WIDTH - PADDLE_WIDTH) / 2.0);
    let bottom_boundary = top_boundary * -1.0;

    ball_transform.translation.x += time.delta_seconds() * PADDLE_SPEED * ball.direction.x;
    ball_transform.translation.y += time.delta_seconds() * PADDLE_SPEED * ball.direction.y;

    let x = ball_transform.translation.x;
    let y = ball_transform.translation.y;

    if x >= top_boundary || x <= bottom_boundary {
        ball_transform.translation.x = ball_transform
            .translation
            .x
            .clamp(bottom_boundary + 1.0, top_boundary - 1.0);
        ball.direction.x *= -1.0;
    }

    if y >= top_boundary || y <= bottom_boundary {
        ball_transform.translation.y = ball_transform
            .translation
            .y
            .clamp(bottom_boundary + 1.0, top_boundary - 1.0);
        ball.direction.y *= -1.0;
    }
}
