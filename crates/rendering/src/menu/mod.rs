pub mod game_over;
pub mod main_menu;
pub mod pause_menu;
pub mod settings_menu;

use bevy::prelude::*;

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((
      main_menu::MainMenuPlugin,
      pause_menu::PauseMenuPlugin,
      game_over::GameOverPlugin,
      settings_menu::SettingsMenuPlugin,
    ));
  }
}
