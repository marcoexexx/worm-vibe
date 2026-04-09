mod cheat_console;
mod leaderboard;
mod minimap;
mod score_display;

pub use cheat_console::{CheatConsole, CheatConsolePlugin};
pub use score_display::ScoreDisplayPlugin;

use bevy::prelude::*;
use leaderboard::LeaderboardPlugin;
use minimap::MinimapPlugin;

pub(crate) struct HudPlugin;

impl Plugin for HudPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((ScoreDisplayPlugin, CheatConsolePlugin, MinimapPlugin, LeaderboardPlugin));
  }
}
