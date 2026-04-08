use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoodKind {
  Donut,
  Cookie,
  Cherry,
  Banana,
  Apple,
  Grape,
  Watermelon,
  Strawberry,
}

impl FoodKind {
  /// All food kinds for random selection.
  pub const ALL: [FoodKind; 8] = [
    Self::Donut,
    Self::Cookie,
    Self::Cherry,
    Self::Banana,
    Self::Apple,
    Self::Grape,
    Self::Watermelon,
    Self::Strawberry,
  ];

  pub fn score_value(self) -> u64 {
    match self {
      Self::Donut => 1,
      Self::Cookie => 2,
      Self::Banana => 1,
      Self::Apple => 2,
      Self::Grape => 1,
      Self::Strawberry => 3,
      Self::Watermelon => 3,
      Self::Cherry => 5,
    }
  }

  pub fn growth(self) -> usize {
    match self {
      Self::Cherry => 2,
      _ => 1,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Food {
  position: Vec2,
  kind: FoodKind,
  radius: f32,
}

impl Food {
  pub fn new(position: Vec2, kind: FoodKind, radius: f32) -> Self {
    Self { position, kind, radius }
  }

  pub fn position(&self) -> Vec2 {
    self.position
  }

  pub fn kind(&self) -> FoodKind {
    self.kind
  }

  pub fn radius(&self) -> f32 {
    self.radius
  }

  pub fn set_position(&mut self, pos: Vec2) {
    self.position = pos;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn food_kind_score_values() {
    // All kinds should have a positive score value
    for kind in FoodKind::ALL {
      assert!(kind.score_value() > 0, "{:?} has zero score", kind);
    }
  }

  #[test]
  fn food_kind_growth_values() {
    for kind in FoodKind::ALL {
      assert!(kind.growth() > 0, "{:?} has zero growth", kind);
    }
    // Cherry gives the most growth
    assert_eq!(FoodKind::Cherry.growth(), 2);
  }

  #[test]
  fn food_construction_and_accessors() {
    let food = Food::new(Vec2::new(10.0, 20.0), FoodKind::Cherry, 11.0);
    assert_eq!(food.position(), Vec2::new(10.0, 20.0));
    assert_eq!(food.kind(), FoodKind::Cherry);
    assert!((food.radius() - 11.0).abs() < f32::EPSILON);
  }

  #[test]
  fn food_set_position() {
    let mut food = Food::new(Vec2::ZERO, FoodKind::Donut, 5.0);
    food.set_position(Vec2::new(99.0, 88.0));
    assert_eq!(food.position(), Vec2::new(99.0, 88.0));
  }

  #[test]
  fn all_kinds_count() {
    assert_eq!(FoodKind::ALL.len(), 8);
  }
}
