use bevy::input::touch::TouchInput;
use bevy::prelude::*;
use domain::MovementIntent;
use glam::Vec2;

use crate::CurrentIntent;

const DEAD_ZONE: f32 = 10.0;
const JOYSTICK_RADIUS: f32 = 80.0;

/// Tracks the virtual joystick state.
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

pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<VirtualJoystick>()
      .add_systems(PreUpdate, read_touch);
  }
}

fn read_touch(
  mut events: EventReader<TouchInput>,
  mut joystick: ResMut<VirtualJoystick>,
  mut intent: ResMut<CurrentIntent>,
  windows: Query<&Window>,
) {
  let window = match windows.get_single() {
    Ok(w) => w,
    Err(_) => return,
  };
  let half_width = window.width() / 2.0;

  for event in events.read() {
    let pos = Vec2::new(event.position.x, event.position.y);

    match event.phase {
      bevy::input::touch::TouchPhase::Started => {
        // Left half: joystick. Right half: boost.
        if pos.x < half_width && joystick.finger_id.is_none() {
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
      }
    }
  }

  // Convert joystick to intent
  if let (Some(center), Some(current)) = (joystick.center, joystick.current) {
    let delta = current - center;
    // Flip Y: screen Y is down, game Y is up
    let game_delta = Vec2::new(delta.x, -delta.y);

    if game_delta.length() > DEAD_ZONE {
      intent.0 = MovementIntent::new(game_delta);
    }
  }
}
