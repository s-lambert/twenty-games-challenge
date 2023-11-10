mod menu;

use bevy::{
    prelude::*,
    window::{self, WindowResolution},
};
use menu::MenuPlugin;
use prelude::GameState;
use rive_bevy::RivePlugin;

pub mod prelude {
    use bevy::prelude::States;

    #[derive(States, Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub enum GameState {
        #[default]
        Menu,
        Pong,
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Twenty Games".to_string(),
                        resolution: WindowResolution::new(500.0, 500.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameState>()
        .add_plugins(RivePlugin)
        .add_systems(Update, window::close_on_esc)
        .add_plugins(MenuPlugin)
        .run()
}
