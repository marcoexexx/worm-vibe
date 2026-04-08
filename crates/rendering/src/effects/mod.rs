pub mod boost;
pub mod death;
pub mod eat;
pub mod screen_shake;

use bevy::prelude::*;

pub(crate) struct EffectsPlugin;

impl Plugin for EffectsPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins((
      death::DeathEffectPlugin,
      eat::EatEffectPlugin,
      screen_shake::ScreenShakePlugin,
      boost::BoostEffectPlugin,
    ));
  }
}
