use domain::Heading;
use glam::Vec2;

/// Advance a worm head forward along its heading at current speed.
pub fn move_head(position: Vec2, heading: f32, speed: f32, dt: f32) -> Vec2 {
  let direction = Vec2::new(heading.cos(), heading.sin());
  position + direction * speed * dt
}

/// A follower segment catches up to its leader, maintaining `spacing`.
/// Uses smooth exponential interpolation for natural-looking curves.
pub fn follow_segment(leader: Vec2, follower: Vec2, spacing: f32, follow_speed: f32, dt: f32) -> Vec2 {
  let delta = leader - follower;
  let dist = delta.length();

  if dist <= spacing {
    return follower;
  }

  let excess = dist - spacing;
  let t = (follow_speed * dt).min(1.0);
  follower + delta.normalize() * excess * t
}

/// Smoothly interpolate heading toward a target, clamped by turn rate.
pub fn smooth_turn(current: f32, target: f32, turn_rate: f32, dt: f32) -> f32 {
  Heading(current).toward(target, turn_rate, dt)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::consts::{FRAC_PI_2, PI};

  #[test]
  fn move_head_right() {
    let pos = Vec2::ZERO;
    let result = move_head(pos, 0.0, 100.0, 1.0);
    assert!((result.x - 100.0).abs() < 0.01);
    assert!(result.y.abs() < 0.01);
  }

  #[test]
  fn move_head_up() {
    let result = move_head(Vec2::ZERO, FRAC_PI_2, 200.0, 0.5);
    assert!(result.x.abs() < 0.01);
    assert!((result.y - 100.0).abs() < 0.01);
  }

  #[test]
  fn move_head_scales_with_dt() {
    let a = move_head(Vec2::ZERO, 0.0, 100.0, 0.1);
    let b = move_head(Vec2::ZERO, 0.0, 100.0, 0.2);
    assert!((b.x - a.x * 2.0).abs() < 0.01);
  }

  #[test]
  fn move_head_zero_speed() {
    let pos = Vec2::new(5.0, 5.0);
    let result = move_head(pos, 0.0, 0.0, 1.0);
    assert_eq!(result, pos);
  }

  #[test]
  fn follow_segment_stays_when_close() {
    let leader = Vec2::new(10.0, 0.0);
    let follower = Vec2::new(5.0, 0.0);
    let spacing = 10.0;
    // Distance (5) < spacing (10), should not move
    let result = follow_segment(leader, follower, spacing, 10.0, 0.1);
    assert_eq!(result, follower);
  }

  #[test]
  fn follow_segment_catches_up() {
    let leader = Vec2::new(100.0, 0.0);
    let follower = Vec2::ZERO;
    let spacing = 10.0;
    let result = follow_segment(leader, follower, spacing, 10.0, 0.1);
    // Should move toward leader
    assert!(result.x > 0.0);
    assert!(result.x < 100.0);
  }

  #[test]
  fn follow_segment_moves_toward_leader_direction() {
    let leader = Vec2::new(0.0, 50.0);
    let follower = Vec2::ZERO;
    let result = follow_segment(leader, follower, 5.0, 20.0, 0.1);
    // Should move in +Y direction
    assert!(result.y > 0.0);
    assert!(result.x.abs() < 0.01);
  }

  #[test]
  fn follow_segment_high_speed_snaps_close() {
    let leader = Vec2::new(30.0, 0.0);
    let follower = Vec2::ZERO;
    let spacing = 5.0;
    // Very high follow speed, should get close to target distance
    let result = follow_segment(leader, follower, spacing, 100.0, 1.0);
    let dist = result.distance(leader);
    assert!((dist - spacing).abs() < 1.0, "dist={} should be near spacing={}", dist, spacing);
  }

  #[test]
  fn smooth_turn_no_change_when_same() {
    let result = smooth_turn(1.0, 1.0, PI, 0.1);
    assert!((result - 1.0).abs() < 0.001);
  }

  #[test]
  fn smooth_turn_moves_toward_target() {
    let result = smooth_turn(0.0, 1.0, PI, 0.1);
    assert!(result > 0.0);
    assert!(result < 1.0);
  }

  #[test]
  fn smooth_turn_clamped_by_rate() {
    let result = smooth_turn(0.0, PI, 0.5, 0.1); // max_turn = 0.05
    assert!((result - 0.05).abs() < 0.001);
  }
}
