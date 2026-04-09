use domain::{Food, FoodConfig, FoodKind};
use glam::Vec2;
use rand::Rng;

/// Spawn a batch of food items within the arena.
/// Each food gets a random kind and random size (70% small, 20% normal, 10% slightly bigger).
pub(crate) fn spawn_batch(half_extents: Vec2, count: usize, cfg: &FoodConfig, rng: &mut impl Rng) -> Vec<Food> {
  (0..count)
    .map(|_| {
      let position = Vec2::new(
        rng.gen_range(-half_extents.x..half_extents.x),
        rng.gen_range(-half_extents.y..half_extents.y),
      );
      let kind = pick_random_kind(rng);
      let radius = random_food_radius(cfg.radius, rng);
      Food::new(position, kind, radius)
    })
    .collect()
}

/// Drop food at each segment position of a dead worm.
/// The dead worm's score is distributed evenly across all dropped food items.
pub(crate) fn drop_from_segments(
  positions: &[Vec2],
  worm_radius: f32,
  worm_score: u64,
  cfg: &FoodConfig,
  rng: &mut impl Rng,
) -> Vec<Food> {
  if positions.is_empty() {
    return Vec::new();
  }

  let jitter = cfg.death_drop_jitter;
  let base_radius = (worm_radius * 0.8).max(cfg.radius);

  // Distribute dead worm's score across all dropped food
  let count = positions.len() as u64;
  let bonus_per_food = worm_score / count.max(1);
  let remainder = worm_score % count.max(1);

  positions
    .iter()
    .enumerate()
    .map(|(i, &pos)| {
      let scatter = Vec2::new(rng.gen_range(-jitter..jitter), rng.gen_range(-jitter..jitter));
      let kind = pick_random_kind(rng);
      let radius = random_food_radius(base_radius, rng);
      // First food gets any remainder score
      let bonus = bonus_per_food + if (i as u64) < remainder { 1 } else { 0 };
      Food::new(pos + scatter, kind, radius).with_bonus_score(bonus)
    })
    .collect()
}

/// Pick a random food kind (uniform distribution across all types).
fn pick_random_kind(rng: &mut impl Rng) -> FoodKind {
  let kinds = FoodKind::ALL;
  kinds[rng.gen_range(0..kinds.len())]
}

/// Random food size: 70% small (0.5-0.7x), 20% normal (0.8-1.0x), 10% slightly bigger (1.0-1.2x).
fn random_food_radius(base: f32, rng: &mut impl Rng) -> f32 {
  let roll: f32 = rng.gen();
  let scale = if roll < 0.70 {
    rng.gen_range(0.5..0.7)
  } else if roll < 0.90 {
    rng.gen_range(0.8..1.0)
  } else {
    rng.gen_range(1.0..1.2)
  };
  base * scale
}
