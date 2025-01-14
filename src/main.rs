pub mod events;
mod systems;
mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

pub const TITLE_NAME: &str = "BALLS: THE GAME";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE_NAME.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_system(trans_to_game_state)
        .add_system(trans_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
