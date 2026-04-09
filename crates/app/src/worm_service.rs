use domain::{CollisionConfig, Worm, WormConfig, WormId};
use glam::Vec2;
use rand::Rng;

/// Spawn a worm at a random edge position within the arena.
pub(crate) fn spawn_at_random_edge(
  id: WormId,
  half_extents: Vec2,
  initial_length: usize,
  worm_cfg: &WormConfig,
  collision_cfg: &CollisionConfig,
  rng: &mut impl Rng,
) -> Worm {
  let margin = collision_cfg.spawn_edge_margin;
  let side = rng.gen_range(0..4);
  let position = match side {
    0 => Vec2::new(
      rng.gen_range(-half_extents.x + margin..half_extents.x - margin),
      -half_extents.y + margin,
    ),
    1 => Vec2::new(
      rng.gen_range(-half_extents.x + margin..half_extents.x - margin),
      half_extents.y - margin,
    ),
    2 => Vec2::new(
      -half_extents.x + margin,
      rng.gen_range(-half_extents.y + margin..half_extents.y - margin),
    ),
    _ => Vec2::new(
      half_extents.x - margin,
      rng.gen_range(-half_extents.y + margin..half_extents.y - margin),
    ),
  };

  let heading = rng.gen_range(0.0..std::f32::consts::TAU);
  Worm::spawn(id, position, heading, initial_length, worm_cfg)
}
