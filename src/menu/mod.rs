use crate::prelude::GameState;
use bevy::{prelude::*, render::render_resource::Extent3d};
use rive_bevy::{GenericEvent, SceneTarget, SpriteEntity, StateMachine};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_animation)
            .add_systems(Update, menu_events_system)
            .add_systems(OnExit(GameState::Menu), remove_menu);
    }
}

#[derive(Component)]
struct Menu;

fn setup_animation(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let mut animation_image = Image::default();

    animation_image.resize(Extent3d {
        width: 500,
        height: 500,
        ..default()
    });

    let animation_image_handle = images.add(animation_image.clone());

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });

    let sprite_entity = commands
        .spawn(SpriteBundle {
            texture: animation_image_handle.clone(),
            ..default()
        })
        .insert(Menu)
        .id();

    let state_machine = StateMachine {
        riv: asset_server.load("twenty-game-menu.riv"),
        ..default()
    };

    commands
        .spawn(state_machine)
        .insert(SceneTarget {
            image: animation_image_handle,
            sprite: SpriteEntity {
                entity: Some(sprite_entity),
            },
            ..default()
        })
        .insert(Menu);
}

fn menu_events_system(
    mut rive_event: EventReader<GenericEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in rive_event.read() {
        if event.name == "Click" {
            game_state.set(GameState::Pong);
        } else {
            info!("Unhandled Rive event: {:?}", event);
        }
    }
}

fn remove_menu(mut commands: Commands, menu_query: Query<Entity, With<Menu>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
