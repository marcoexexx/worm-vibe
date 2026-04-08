use bevy::prelude::*;
use domain::MovementIntent;
use glam::Vec2;

use crate::CurrentIntent;

pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, read_keyboard);
  }
}

fn read_keyboard(keys: Res<ButtonInput<KeyCode>>, mut intent: ResMut<CurrentIntent>) {
  let mut dir = Vec2::ZERO;

  if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
    dir.y += 1.0;
  }
  if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
    dir.y -= 1.0;
  }
  if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
    dir.x -= 1.0;
  }
  if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
    dir.x += 1.0;
  }

  let boost = keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::ShiftLeft);

  let movement = if dir.length_squared() > 0.001 {
    MovementIntent::new(dir)
  } else {
    MovementIntent::idle()
  };

  intent.0 = if boost { movement.with_boost() } else { movement };
}
