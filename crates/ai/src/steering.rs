use glam::Vec2;
use rand::Rng;

/// Returns desired heading to move toward a target.
pub fn steering_seek(current_pos: Vec2, target: Vec2) -> f32 {
  let delta = target - current_pos;
  delta.y.atan2(delta.x)
}

/// Returns desired heading to move away from a threat.
pub fn steering_flee(current_pos: Vec2, threat: Vec2) -> f32 {
  let delta = current_pos - threat;
  delta.y.atan2(delta.x)
}

/// Returns a slightly randomized heading for wandering.
pub fn steering_wander(current_heading: f32, rng: &mut impl Rng) -> f32 {
  let jitter = rng.gen_range(-0.3..0.3);
  current_heading + jitter
}
