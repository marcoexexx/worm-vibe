use domain::FoodKind;
use glam::Vec2;

/// What an AI worm can "see" within its perception radius.
pub struct AiPerception {
  pub self_position: Vec2,
  pub self_heading: f32,
  pub self_length: usize,
  pub nearby_food: Vec<(Vec2, FoodKind)>,
  pub nearby_worms: Vec<NearbyWorm>,
  /// Body segments of other worms that are close and in front of us.
  pub nearby_segments: Vec<Vec2>,
  pub arena_half_extents: Vec2,
}

pub struct NearbyWorm {
  pub position: Vec2,
  pub length: usize,
  pub heading: f32,
}
