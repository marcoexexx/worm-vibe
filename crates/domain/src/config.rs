use crate::ArenaConfig;

// ============================================================================
// Worm physics
// ============================================================================

#[derive(Debug, Clone)]
pub struct WormConfig {
  pub base_radius: f32,
  pub base_speed: f32,
  pub max_speed: f32,
  pub acceleration: f32,
  pub deceleration: f32,
  pub boost_multiplier: f32,
  pub boost_drain_interval: f32,
  pub min_segments_for_boost: usize,
  pub segment_spawn_spacing: f32,
  pub segment_follow_spacing: f32,
  pub follow_speed: f32,
  pub initial_speed_fraction: f32,
  /// Speed bonus = ln(length) * this value
  pub length_speed_ln_factor: f32,
  /// Minimum length before speed bonus kicks in
  pub length_speed_threshold: usize,
  /// Radius growth factor: radius = base_radius * (1 + growth_factor * ln(length))
  pub radius_growth_factor: f32,
  /// Maximum radius multiplier cap (e.g. 2.5 = max 2.5× base radius)
  pub max_radius_multiplier: f32,
}

impl Default for WormConfig {
  fn default() -> Self {
    Self {
      base_radius: 14.0,
      base_speed: 280.0,
      max_speed: 320.0,
      acceleration: 400.0,
      deceleration: 600.0,
      boost_multiplier: 1.8,
      boost_drain_interval: 0.4,
      min_segments_for_boost: 5,
      segment_spawn_spacing: 16.0,
      segment_follow_spacing: 14.0,
      follow_speed: 50.0,
      initial_speed_fraction: 0.5,
      length_speed_ln_factor: 8.0,
      length_speed_threshold: 3,
      radius_growth_factor: 0.10,
      max_radius_multiplier: 2.0,
    }
  }
}

// ============================================================================
// Food
// ============================================================================

#[derive(Debug, Clone)]
pub struct FoodConfig {
  pub radius: f32,
  pub max_spawn_per_tick: usize,
  pub death_drop_jitter: f32,
  // Spawn probabilities (normal spawning)
  pub spawn_donut_chance: f32,
  pub spawn_cookie_chance: f32,
  // Death drop thresholds (worm length brackets)
  pub death_tiny_max: usize,
  pub death_small_max: usize,
  pub death_medium_max: usize,
  // Magnetic pull
  pub magnet_range_multiplier: f32,
  pub magnet_pull_speed: f32,
}

impl Default for FoodConfig {
  fn default() -> Self {
    Self {
      radius: 11.0,
      max_spawn_per_tick: 10,
      death_drop_jitter: 6.0,
      spawn_donut_chance: 0.60,
      spawn_cookie_chance: 0.90, // cookie = 0.90 - 0.60 = 30%, cherry = 10%
      death_tiny_max: 10,
      death_small_max: 30,
      death_medium_max: 60,
      magnet_range_multiplier: 6.0,
      magnet_pull_speed: 500.0,
    }
  }
}

// ============================================================================
// Collision
// ============================================================================

#[derive(Debug, Clone)]
pub struct CollisionConfig {
  pub grid_cell_size: f32,
  pub self_segment_skip: usize,
  pub other_segment_skip: usize,
  pub spawn_edge_margin: f32,
  pub direction_threshold: f32,
  /// Shrink factor for worm-vs-worm collision radii (0.0–1.0).
  /// Visual sprites are larger than hitboxes so collisions feel fair.
  pub worm_hitbox_shrink: f32,
}

impl Default for CollisionConfig {
  fn default() -> Self {
    Self {
      grid_cell_size: 60.0,
      self_segment_skip: 8,
      other_segment_skip: 2,
      spawn_edge_margin: 200.0,
      direction_threshold: 0.1,
      worm_hitbox_shrink: 0.75,
    }
  }
}

// ============================================================================
// AI difficulty
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiDifficulty {
  Noob,
  Easy,
  Normal,
  Hard,
  Ruthless,
}

#[derive(Debug, Clone)]
pub struct AiTuning {
  pub perception_radius: f32,
  pub decision_interval: f32,
  pub danger_radius: f32,
  pub hunt_ratio: f32,
  pub uses_boost: bool,
  pub wander_jitter: f32,
  pub predicts_movement: bool,
  pub boundary_margin: f32,
  pub intercept_lookahead: f32,
  pub boost_chase_distance: f32,
}

impl AiDifficulty {
  pub fn tuning(self) -> AiTuning {
    match self {
      Self::Noob => AiTuning {
        perception_radius: 150.0,
        decision_interval: 0.5,
        danger_radius: 80.0,
        hunt_ratio: 99.0,
        uses_boost: false,
        wander_jitter: 0.8,
        predicts_movement: false,
        boundary_margin: 100.0,
        intercept_lookahead: 0.0,
        boost_chase_distance: 0.0,
      },
      Self::Easy => AiTuning {
        perception_radius: 250.0,
        decision_interval: 0.25,
        danger_radius: 120.0,
        hunt_ratio: 4.0,
        uses_boost: false,
        wander_jitter: 0.5,
        predicts_movement: false,
        boundary_margin: 150.0,
        intercept_lookahead: 0.0,
        boost_chase_distance: 0.0,
      },
      Self::Normal => AiTuning {
        perception_radius: 350.0,
        decision_interval: 0.12,
        danger_radius: 160.0,
        hunt_ratio: 2.5,
        uses_boost: false,
        wander_jitter: 0.3,
        predicts_movement: false,
        boundary_margin: 200.0,
        intercept_lookahead: 0.0,
        boost_chase_distance: 0.0,
      },
      Self::Hard => AiTuning {
        perception_radius: 500.0,
        decision_interval: 0.06,
        danger_radius: 220.0,
        hunt_ratio: 1.8,
        uses_boost: true,
        wander_jitter: 0.15,
        predicts_movement: true,
        boundary_margin: 300.0,
        intercept_lookahead: 100.0,
        boost_chase_distance: 200.0,
      },
      Self::Ruthless => AiTuning {
        perception_radius: 700.0,
        decision_interval: 0.03,
        danger_radius: 300.0,
        hunt_ratio: 1.3,
        uses_boost: true,
        wander_jitter: 0.1,
        predicts_movement: true,
        boundary_margin: 400.0,
        intercept_lookahead: 150.0,
        boost_chase_distance: 250.0,
      },
    }
  }
}

// ============================================================================
// Cheats
// ============================================================================

#[derive(Debug, Clone)]
pub struct CheatDefaults {
  pub invincibility_duration: f32,
  pub grow_amount: usize,
  pub speed_multiplier: f32,
  pub speed_duration: f32,
  pub noclip_duration: f32,
  pub score_multiplier: f32,
  pub score_duration: f32,
}

impl Default for CheatDefaults {
  fn default() -> Self {
    Self {
      invincibility_duration: 30.0,
      grow_amount: 50,
      speed_multiplier: 2.0,
      speed_duration: 20.0,
      noclip_duration: 15.0,
      score_multiplier: 5.0,
      score_duration: 30.0,
    }
  }
}

// ============================================================================
// Game preset
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyPreset {
  Easy,
  Normal,
  Hard,
}

#[derive(Debug, Clone)]
pub struct GameConfig {
  pub(crate) arena: ArenaConfig,
  pub(crate) initial_worm_length: usize,
  pub(crate) turn_rate: f32,
  pub(crate) ai_difficulty_mix: Vec<AiDifficulty>,
  pub(crate) worm: WormConfig,
  pub(crate) food: FoodConfig,
  pub(crate) collision: CollisionConfig,
  pub(crate) cheats: CheatDefaults,
}

impl GameConfig {
  pub fn from_preset(preset: DifficultyPreset) -> Self {
    let worm = WormConfig::default();
    let food = FoodConfig::default();
    let collision = CollisionConfig::default();
    let cheats = CheatDefaults::default();

    match preset {
      DifficultyPreset::Easy => Self {
        arena: ArenaConfig::new(4000.0, 4000.0)
          .with_food_density(0.000008)
          .with_max_ai_worms(12),
        initial_worm_length: 5,
        turn_rate: std::f32::consts::PI * 1.2,
        ai_difficulty_mix: vec![
          AiDifficulty::Noob,
          AiDifficulty::Noob,
          AiDifficulty::Noob,
          AiDifficulty::Easy,
          AiDifficulty::Easy,
          AiDifficulty::Easy,
          AiDifficulty::Normal,
          AiDifficulty::Normal,
          AiDifficulty::Normal,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Ruthless,
        ],
        worm,
        food,
        collision,
        cheats,
      },
      DifficultyPreset::Normal => Self {
        arena: ArenaConfig::new(5000.0, 5000.0)
          .with_food_density(0.000006)
          .with_max_ai_worms(20),
        initial_worm_length: 3,
        turn_rate: std::f32::consts::PI * 1.2,
        ai_difficulty_mix: vec![
          AiDifficulty::Noob,
          AiDifficulty::Noob,
          AiDifficulty::Easy,
          AiDifficulty::Easy,
          AiDifficulty::Easy,
          AiDifficulty::Normal,
          AiDifficulty::Normal,
          AiDifficulty::Normal,
          AiDifficulty::Normal,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
        ],
        worm,
        food,
        collision,
        cheats,
      },
      DifficultyPreset::Hard => Self {
        arena: ArenaConfig::new(6000.0, 6000.0)
          .with_food_density(0.000004)
          .with_max_ai_worms(30),
        initial_worm_length: 3,
        turn_rate: std::f32::consts::PI * 1.0,
        ai_difficulty_mix: vec![
          AiDifficulty::Easy,
          AiDifficulty::Normal,
          AiDifficulty::Normal,
          AiDifficulty::Normal,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Hard,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
          AiDifficulty::Ruthless,
        ],
        worm,
        food,
        collision,
        cheats,
      },
    }
  }

  pub fn arena(&self) -> &ArenaConfig {
    &self.arena
  }

  pub fn initial_worm_length(&self) -> usize {
    self.initial_worm_length
  }

  pub fn turn_rate(&self) -> f32 {
    self.turn_rate
  }

  pub fn worm(&self) -> &WormConfig {
    &self.worm
  }

  pub fn food(&self) -> &FoodConfig {
    &self.food
  }

  pub fn collision(&self) -> &CollisionConfig {
    &self.collision
  }

  pub fn cheats(&self) -> &CheatDefaults {
    &self.cheats
  }

  pub fn ai_difficulty_for(&self, index: usize) -> AiDifficulty {
    if self.ai_difficulty_mix.is_empty() {
      return AiDifficulty::Normal;
    }
    self.ai_difficulty_mix[index % self.ai_difficulty_mix.len()]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn easy_preset_has_12_ai_worms() {
    let config = GameConfig::from_preset(DifficultyPreset::Easy);
    assert_eq!(config.arena().max_ai_worms(), 12);
  }

  #[test]
  fn normal_preset_has_20_ai_worms() {
    let config = GameConfig::from_preset(DifficultyPreset::Normal);
    assert_eq!(config.arena().max_ai_worms(), 20);
  }

  #[test]
  fn hard_preset_has_30_ai_worms() {
    let config = GameConfig::from_preset(DifficultyPreset::Hard);
    assert_eq!(config.arena().max_ai_worms(), 30);
  }

  #[test]
  fn ai_difficulty_wraps_around() {
    let config = GameConfig::from_preset(DifficultyPreset::Easy);
    let mix_len = config.ai_difficulty_mix.len();
    let d0 = config.ai_difficulty_for(0);
    let d_wrap = config.ai_difficulty_for(mix_len);
    assert_eq!(d0, d_wrap);
  }

  #[test]
  fn worm_config_defaults_are_sane() {
    let cfg = WormConfig::default();
    assert!(cfg.base_speed > 0.0);
    assert!(cfg.max_speed >= cfg.base_speed);
    assert!(cfg.boost_multiplier > 1.0);
    assert!(cfg.min_segments_for_boost > 0);
  }

  #[test]
  fn food_config_probabilities_are_valid() {
    let cfg = FoodConfig::default();
    assert!(cfg.spawn_donut_chance > 0.0 && cfg.spawn_donut_chance < 1.0);
    assert!(cfg.spawn_cookie_chance > cfg.spawn_donut_chance);
    assert!(cfg.spawn_cookie_chance <= 1.0);
  }

  #[test]
  fn all_ai_difficulties_have_positive_perception() {
    let difficulties = [
      AiDifficulty::Noob,
      AiDifficulty::Easy,
      AiDifficulty::Normal,
      AiDifficulty::Hard,
      AiDifficulty::Ruthless,
    ];
    for d in difficulties {
      let tuning = d.tuning();
      assert!(tuning.perception_radius > 0.0, "{:?} has zero perception", d);
      assert!(tuning.decision_interval > 0.0, "{:?} has zero interval", d);
    }
  }

  #[test]
  fn harder_difficulties_have_larger_perception() {
    let easy = AiDifficulty::Easy.tuning();
    let hard = AiDifficulty::Hard.tuning();
    let ruthless = AiDifficulty::Ruthless.tuning();
    assert!(hard.perception_radius > easy.perception_radius);
    assert!(ruthless.perception_radius > hard.perception_radius);
  }

  #[test]
  fn preset_arena_sizes_increase_with_difficulty() {
    let easy = GameConfig::from_preset(DifficultyPreset::Easy);
    let normal = GameConfig::from_preset(DifficultyPreset::Normal);
    let hard = GameConfig::from_preset(DifficultyPreset::Hard);
    assert!(normal.arena().width() > easy.arena().width());
    assert!(hard.arena().width() > normal.arena().width());
  }
}
