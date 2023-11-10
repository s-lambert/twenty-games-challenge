mod menu;
mod pong;

use bevy::{
    prelude::*,
    window::{self, WindowResolution},
};
use menu::MenuPlugin;
use pong::PongPlugin;
use prelude::*;
use rive_bevy::RivePlugin;

pub mod prelude {
    use bevy::prelude::States;

    #[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub enum GameState {
        #[default]
        Menu,
        Pong,
    }

    pub const WINDOW_HEIGHT: f32 = 500.0;
    pub const WINDOW_WIDTH: f32 = 500.0;
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Twenty Games".to_string(),
                        resolution: WindowResolution::new(WINDOW_HEIGHT, WINDOW_WIDTH),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameState>()
        .add_plugins(RivePlugin)
        .add_systems(Update, window::close_on_esc)
        .add_plugins(MenuPlugin)
        .add_plugins(PongPlugin)
        .run()
}
