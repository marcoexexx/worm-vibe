use glam::Vec2;

/// Normalized direction the player intends to move.
/// `None` means no input (worm continues straight).
#[derive(Debug, Clone, Copy, Default)]
pub struct MovementIntent {
  direction: Option<Vec2>,
  boost: bool,
}

impl MovementIntent {
  pub fn new(direction: Vec2) -> Self {
    let normalized = if direction.length_squared() > 0.001 {
      Some(direction.normalize())
    } else {
      None
    };
    Self {
      direction: normalized,
      boost: false,
    }
  }

  pub fn with_boost(mut self) -> Self {
    self.boost = true;
    self
  }

  pub fn idle() -> Self {
    Self::default()
  }

  pub fn direction(&self) -> Option<Vec2> {
    self.direction
  }

  pub fn wants_boost(&self) -> bool {
    self.boost
  }

  /// Convert direction to target heading in radians.
  pub fn target_heading(&self) -> Option<f32> {
    self.direction.map(|d| d.y.atan2(d.x))
  }
}

/// Current heading in radians, wrapping utilities.
#[derive(Debug, Clone, Copy)]
pub struct Heading(pub f32);

impl Heading {
  pub fn toward(self, target: f32, turn_rate: f32, dt: f32) -> f32 {
    let mut delta = target - self.0;

    // Normalize to [-PI, PI]
    while delta > std::f32::consts::PI {
      delta -= std::f32::consts::TAU;
    }
    while delta < -std::f32::consts::PI {
      delta += std::f32::consts::TAU;
    }

    let max_turn = turn_rate * dt;
    self.0 + delta.clamp(-max_turn, max_turn)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::consts::{FRAC_PI_2, PI};

  #[test]
  fn movement_intent_idle_has_no_direction() {
    let intent = MovementIntent::idle();
    assert!(intent.direction().is_none());
    assert!(intent.target_heading().is_none());
    assert!(!intent.wants_boost());
  }

  #[test]
  fn movement_intent_normalizes_direction() {
    let intent = MovementIntent::new(Vec2::new(3.0, 4.0));
    let dir = intent.direction().unwrap();
    assert!((dir.length() - 1.0).abs() < 0.001);
  }

  #[test]
  fn movement_intent_zero_vector_becomes_idle() {
    let intent = MovementIntent::new(Vec2::ZERO);
    assert!(intent.direction().is_none());
  }

  #[test]
  fn movement_intent_with_boost() {
    let intent = MovementIntent::new(Vec2::X).with_boost();
    assert!(intent.wants_boost());
  }

  #[test]
  fn target_heading_right_is_zero() {
    let intent = MovementIntent::new(Vec2::X);
    let heading = intent.target_heading().unwrap();
    assert!(heading.abs() < 0.01);
  }

  #[test]
  fn target_heading_up_is_pi_over_2() {
    let intent = MovementIntent::new(Vec2::Y);
    let heading = intent.target_heading().unwrap();
    assert!((heading - FRAC_PI_2).abs() < 0.01);
  }

  #[test]
  fn heading_toward_same_direction() {
    let heading = Heading(0.0);
    let result = heading.toward(0.0, PI, 1.0);
    assert!(result.abs() < 0.001);
  }

  #[test]
  fn heading_toward_clamps_to_turn_rate() {
    let heading = Heading(0.0);
    let result = heading.toward(PI, 1.0, 0.1); // max_turn = 0.1
    assert!((result - 0.1).abs() < 0.001);
  }

  #[test]
  fn heading_toward_wraps_around() {
    // From nearly PI to nearly -PI should turn the short way
    let heading = Heading(PI - 0.1);
    let result = heading.toward(-PI + 0.1, 10.0, 1.0);
    // Should go through PI, not all the way around
    assert!(result > PI - 0.5 || result < -PI + 0.5);
  }
}
