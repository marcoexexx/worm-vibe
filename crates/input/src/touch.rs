use bevy::input::touch::TouchInput;
use bevy::prelude::*;
use domain::{ControlMode, MovementIntent};
use glam::Vec2;

use crate::{ActiveControlMode, CurrentIntent};

const DEAD_ZONE: f32 = 10.0;
const JOYSTICK_RADIUS: f32 = 80.0;

#[derive(Resource, Default)]
pub struct VirtualJoystick {
  center: Option<Vec2>,
  current: Option<Vec2>,
  finger_id: Option<u64>,
}

impl VirtualJoystick {
  pub fn center(&self) -> Option<Vec2> {
    self.center
  }

  pub fn current(&self) -> Option<Vec2> {
    self.current
  }

  pub fn radius(&self) -> f32 {
    JOYSTICK_RADIUS
  }

  pub fn is_active(&self) -> bool {
    self.center.is_some()
  }
}

/// Tracks whether the boost button is being held.
#[derive(Resource, Default)]
pub struct TouchBoost {
  finger_id: Option<u64>,
  pub active: bool,
}

pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<VirtualJoystick>()
      .init_resource::<TouchBoost>()
      .add_systems(PreUpdate, read_touch.run_if(is_joystick_mode));
  }
}

fn is_joystick_mode(mode: Res<ActiveControlMode>) -> bool {
  mode.0 == ControlMode::Joystick
}

fn read_touch(
  mut events: EventReader<TouchInput>,
  mut joystick: ResMut<VirtualJoystick>,
  mut boost: ResMut<TouchBoost>,
  mut intent: ResMut<CurrentIntent>,
  windows: Query<&Window>,
) {
  let Ok(window) = windows.get_single() else {
    return;
  };
  let half_width = window.width() / 2.0;

  for event in events.read() {
    let pos = Vec2::new(event.position.x, event.position.y);

    match event.phase {
      bevy::input::touch::TouchPhase::Started => {
        if pos.x < half_width && joystick.finger_id.is_none() {
          // Left side: boost button area (bottom-left quadrant)
          if pos.y > window.height() * 0.6 && pos.x < window.width() * 0.25 {
            boost.finger_id = Some(event.id);
            boost.active = true;
          } else {
            // Left side: joystick
            joystick.finger_id = Some(event.id);
            joystick.center = Some(pos);
            joystick.current = Some(pos);
          }
        } else if pos.x >= half_width && joystick.finger_id.is_none() {
          // Right side: joystick
          joystick.finger_id = Some(event.id);
          joystick.center = Some(pos);
          joystick.current = Some(pos);
        }
      }
      bevy::input::touch::TouchPhase::Moved => {
        if joystick.finger_id == Some(event.id) {
          joystick.current = Some(pos);
        }
      }
      bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
        if joystick.finger_id == Some(event.id) {
          joystick.finger_id = None;
          joystick.center = None;
          joystick.current = None;
        }
        if boost.finger_id == Some(event.id) {
          boost.finger_id = None;
          boost.active = false;
        }
      }
    }
  }

  // Convert joystick to intent
  if let (Some(center), Some(current)) = (joystick.center, joystick.current) {
    let delta = current - center;
    let game_delta = Vec2::new(delta.x, -delta.y);

    if game_delta.length() > DEAD_ZONE {
      let movement = MovementIntent::new(game_delta);
      intent.0 = if boost.active { movement.with_boost() } else { movement };
    }
  }
}
