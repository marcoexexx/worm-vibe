mod button_hover;
mod cheat_handler;
mod effects_handler;
mod game_tick;
mod menu_handler;
mod persistence;

use bevy::prelude::*;

pub(crate) struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((
      game_tick::GameTickPlugin,
      menu_handler::MenuHandlerPlugin,
      cheat_handler::CheatHandlerPlugin,
      persistence::PersistencePlugin,
      button_hover::ButtonHoverPlugin,
      effects_handler::EffectsHandlerPlugin,
    ));
  }
}
