use bevy::prelude::*;
use domain::{ControlMode, MovementIntent};

use crate::{ActiveControlMode, CurrentIntent};

pub struct MouseInputPlugin;

impl Plugin for MouseInputPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, read_mouse.run_if(is_mouse_mode));
  }
}

fn is_mouse_mode(mode: Res<ActiveControlMode>) -> bool {
  mode.0 == ControlMode::Mouse
}

fn read_mouse(
  windows: Query<&Window>,
  cameras: Query<(&Camera, &GlobalTransform)>,
  keys: Res<ButtonInput<KeyCode>>,
  mouse_buttons: Res<ButtonInput<MouseButton>>,
  mut intent: ResMut<CurrentIntent>,
) {
  let Ok(window) = windows.get_single() else {
    return;
  };
  let Some(cursor_pos) = window.cursor_position() else {
    return;
  };
  let Ok((camera, camera_transform)) = cameras.get_single() else {
    return;
  };

  // Convert screen cursor to world position
  let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
    return;
  };

  // Player head is at camera center (camera follows player)
  let player_pos = camera_transform.translation().truncate();
  let delta = world_pos - player_pos;

  let boost =
    keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::ShiftLeft) || mouse_buttons.pressed(MouseButton::Right);

  let movement = if delta.length_squared() > 25.0 {
    // 5px dead zone
    MovementIntent::new(delta)
  } else {
    MovementIntent::idle()
  };

  intent.0 = if boost { movement.with_boost() } else { movement };
}
