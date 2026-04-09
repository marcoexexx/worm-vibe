use glam::Vec2;

#[derive(Debug, Clone)]
pub struct ArenaConfig {
  width: f32,
  height: f32,
  food_density: f32,
  max_ai_worms: usize,
}

impl ArenaConfig {
  pub fn new(width: f32, height: f32) -> Self {
    Self {
      width,
      height,
      food_density: 0.0002,
      max_ai_worms: 8,
    }
  }

  pub fn with_food_density(mut self, density: f32) -> Self {
    self.food_density = density;
    self
  }

  pub fn with_max_ai_worms(mut self, count: usize) -> Self {
    self.max_ai_worms = count;
    self
  }

  pub fn width(&self) -> f32 {
    self.width
  }

  pub fn height(&self) -> f32 {
    self.height
  }

  pub fn target_food_count(&self) -> usize {
    (self.width * self.height * self.food_density) as usize
  }

  pub fn max_ai_worms(&self) -> usize {
    self.max_ai_worms
  }

  pub fn half_extents(&self) -> Vec2 {
    Vec2::new(self.width / 2.0, self.height / 2.0)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ArenaBounds {
  pub min: Vec2,
  pub max: Vec2,
}

impl From<&ArenaConfig> for ArenaBounds {
  fn from(config: &ArenaConfig) -> Self {
    let half = config.half_extents();
    Self { min: -half, max: half }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn arena_half_extents() {
    let arena = ArenaConfig::new(1000.0, 800.0);
    let half = arena.half_extents();
    assert_eq!(half, Vec2::new(500.0, 400.0));
  }

  #[test]
  fn arena_target_food_count() {
    let arena = ArenaConfig::new(1000.0, 1000.0).with_food_density(0.001);
    // 1000 * 1000 * 0.001 = 1000
    assert_eq!(arena.target_food_count(), 1000);
  }

  #[test]
  fn arena_bounds_from_config() {
    let arena = ArenaConfig::new(200.0, 100.0);
    let bounds = ArenaBounds::from(&arena);
    assert_eq!(bounds.min, Vec2::new(-100.0, -50.0));
    assert_eq!(bounds.max, Vec2::new(100.0, 50.0));
  }

  #[test]
  fn arena_builder_methods() {
    let arena = ArenaConfig::new(500.0, 500.0)
      .with_food_density(0.01)
      .with_max_ai_worms(12);
    assert_eq!(arena.max_ai_worms(), 12);
    assert_eq!(arena.width(), 500.0);
    assert_eq!(arena.height(), 500.0);
  }
}
