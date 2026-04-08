use crate::perception::{AiPerception, NearbyWorm};
use crate::steering::{steering_flee, steering_seek};
use domain::{AiDifficulty, AiTuning};
use glam::Vec2;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum AiState {
  Foraging,
  Fleeing,
  Hunting,
}

#[derive(Debug, Clone)]
pub enum AiDecision {
  TurnTo(f32),
  Boost,
}

pub trait AiBrain {
  fn decide(&mut self, perception: &AiPerception, rng: &mut impl Rng) -> Vec<AiDecision>;
}

pub struct BasicAiBrain {
  difficulty: AiDifficulty,
  tuning: AiTuning,
  state: AiState,
  timer: f32,
  cached_decisions: Vec<AiDecision>,
}

impl BasicAiBrain {
  pub fn new() -> Self {
    Self::with_difficulty(AiDifficulty::Normal)
  }

  pub fn with_difficulty(difficulty: AiDifficulty) -> Self {
    let tuning = difficulty.tuning();
    Self {
      difficulty,
      tuning,
      state: AiState::Foraging,
      timer: 0.0,
      cached_decisions: Vec::new(),
    }
  }

  pub fn difficulty(&self) -> AiDifficulty {
    self.difficulty
  }

  pub fn perception_radius(&self) -> f32 {
    self.tuning.perception_radius
  }

  pub fn update(&mut self, dt: f32) {
    self.timer += dt;
  }

  fn find_threat<'a>(&self, perception: &'a AiPerception) -> Option<&'a NearbyWorm> {
    let danger_r = self.tuning.danger_radius;
    perception.nearby_worms.iter().find(|w| {
      let dist = w.position.distance(perception.self_position);
      dist < danger_r && w.length > perception.self_length
    })
  }

  fn find_prey<'a>(&self, perception: &'a AiPerception) -> Option<&'a NearbyWorm> {
    let ratio = self.tuning.hunt_ratio;
    perception
      .nearby_worms
      .iter()
      .filter(|w| perception.self_length as f32 > w.length as f32 * ratio)
      .min_by(|a, b| {
        let da = a.position.distance_squared(perception.self_position);
        let db = b.position.distance_squared(perception.self_position);
        da.partial_cmp(&db).unwrap()
      })
  }

  fn find_best_food(&self, perception: &AiPerception) -> Option<Vec2> {
    if perception.nearby_food.is_empty() {
      return None;
    }

    // Smarter AI considers food value, not just distance
    if self.tuning.predicts_movement {
      // Value-weighted: prefer high-value food nearby
      perception
        .nearby_food
        .iter()
        .min_by(|a, b| {
          let da = a.0.distance_squared(perception.self_position);
          let db = b.0.distance_squared(perception.self_position);
          let va = da / (a.1.score_value() as f32 + 1.0);
          let vb = db / (b.1.score_value() as f32 + 1.0);
          va.partial_cmp(&vb).unwrap()
        })
        .map(|(pos, _)| *pos)
    } else {
      // Simple: nearest food
      perception
        .nearby_food
        .iter()
        .min_by(|a, b| {
          let da = a.0.distance_squared(perception.self_position);
          let db = b.0.distance_squared(perception.self_position);
          da.partial_cmp(&db).unwrap()
        })
        .map(|(pos, _)| *pos)
    }
  }

  fn intercept_heading(&self, perception: &AiPerception, prey: &NearbyWorm) -> f32 {
    if self.tuning.predicts_movement {
      // Predict where prey will be in ~0.5 seconds
      let prey_dir = Vec2::new(prey.heading.cos(), prey.heading.sin());
      let predicted = prey.position + prey_dir * 100.0;
      steering_seek(perception.self_position, predicted)
    } else {
      steering_seek(perception.self_position, prey.position)
    }
  }
}

impl AiBrain for BasicAiBrain {
  fn decide(&mut self, perception: &AiPerception, rng: &mut impl Rng) -> Vec<AiDecision> {
    let interval = self.tuning.decision_interval;
    if self.timer < interval {
      return self.cached_decisions.clone();
    }
    self.timer = 0.0;

    let mut decisions = Vec::new();

    // State transitions
    if self.find_threat(perception).is_some() {
      self.state = AiState::Fleeing;
    } else if self.find_prey(perception).is_some() {
      self.state = AiState::Hunting;
    } else {
      self.state = AiState::Foraging;
    }

    match self.state {
      AiState::Fleeing => {
        if let Some(threat) = self.find_threat(perception) {
          let heading = steering_flee(perception.self_position, threat.position);
          decisions.push(AiDecision::TurnTo(heading));
          // Smart AI boosts to escape
          if self.tuning.uses_boost {
            decisions.push(AiDecision::Boost);
          }
        }
      }
      AiState::Hunting => {
        if let Some(prey) = self.find_prey(perception) {
          let heading = self.intercept_heading(perception, prey);
          decisions.push(AiDecision::TurnTo(heading));
          // Aggressive AI boosts to catch prey
          if self.tuning.uses_boost {
            let dist = prey.position.distance(perception.self_position);
            if dist < self.tuning.boost_chase_distance {
              decisions.push(AiDecision::Boost);
            }
          }
        }
      }
      AiState::Foraging => {
        if let Some(food_pos) = self.find_best_food(perception) {
          let heading = steering_seek(perception.self_position, food_pos);
          decisions.push(AiDecision::TurnTo(heading));
        } else {
          // Wander: apply a small random offset to the current heading with strong
          // forward bias so the worm doesn't spin in tight circles.
          let jitter = self.tuning.wander_jitter;
          // Bias weight keeps the worm mostly going straight; jitter adds gentle curves.
          let random_offset: f32 = rng.gen_range(-jitter..jitter);
          let heading = perception.self_heading + random_offset * 0.4;
          decisions.push(AiDecision::TurnTo(heading));
        }
      }
    }

    // Avoid arena boundaries
    let pos = perception.self_position;
    let half = perception.arena_half_extents;
    let margin = self.tuning.boundary_margin;
    if pos.x.abs() > half.x - margin || pos.y.abs() > half.y - margin {
      let center_heading = steering_seek(pos, Vec2::ZERO);
      decisions.clear();
      decisions.push(AiDecision::TurnTo(center_heading));
    }

    self.cached_decisions = decisions.clone();
    decisions
  }
}

impl Default for BasicAiBrain {
  fn default() -> Self {
    Self::new()
  }
}
