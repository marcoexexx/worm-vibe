mod keyboard;
mod mouse;
mod touch;

pub use keyboard::KeyboardInputPlugin;
pub use mouse::MouseInputPlugin;
pub use touch::{TouchInputPlugin, VirtualJoystick};

use bevy::prelude::*;
use domain::{ControlMode, MovementIntent};

/// Shared resource: the current frame's movement intent.
#[derive(Resource, Default)]
pub struct CurrentIntent(pub MovementIntent);

/// Which control mode is active — synced from game settings.
#[derive(Resource, Default)]
pub struct ActiveControlMode(pub ControlMode);

/// Input plugin — registers all input systems, gated by active control mode.
pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<CurrentIntent>()
      .init_resource::<ActiveControlMode>()
      .add_plugins((KeyboardInputPlugin, TouchInputPlugin, MouseInputPlugin));
  }
}
