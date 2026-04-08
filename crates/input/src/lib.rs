mod keyboard;
mod touch;

pub use keyboard::KeyboardInputPlugin;
pub use touch::{TouchInputPlugin, VirtualJoystick};

use bevy::prelude::*;
use domain::MovementIntent;

/// Shared resource: the current frame's movement intent.
#[derive(Resource, Default)]
pub struct CurrentIntent(pub MovementIntent);

/// Input plugin — registers keyboard + touch systems.
pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<CurrentIntent>()
      .add_plugins((KeyboardInputPlugin, TouchInputPlugin));
  }
}
