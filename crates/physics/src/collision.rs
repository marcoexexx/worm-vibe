use domain::{ArenaBounds, WormId};
use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionEntity {
  WormHead(WormId),
  WormSegment(WormId, usize),
  Food(usize),
}

#[derive(Debug, Clone)]
pub enum CollisionResult {
  WormAteFood { worm_id: WormId, food_index: usize },
  WormHitWorm { victim: WormId, killer: WormId },
  WormHitBoundary { worm_id: WormId },
}

/// Circle-circle overlap test.
pub fn check_circle_overlap(a_pos: Vec2, a_radius: f32, b_pos: Vec2, b_radius: f32) -> bool {
  let combined = a_radius + b_radius;
  a_pos.distance_squared(b_pos) < combined * combined
}

/// Check whether a circle is outside the arena bounds.
pub fn check_boundary(pos: Vec2, radius: f32, bounds: &ArenaBounds) -> bool {
  pos.x - radius < bounds.min.x
    || pos.x + radius > bounds.max.x
    || pos.y - radius < bounds.min.y
    || pos.y + radius > bounds.max.y
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn overlap_touching_circles() {
    // Two circles with combined radius 20, distance 19 — overlapping
    assert!(check_circle_overlap(Vec2::ZERO, 10.0, Vec2::new(19.0, 0.0), 10.0));
  }

  #[test]
  fn no_overlap_separated_circles() {
    // Distance 25 > combined radius 20
    assert!(!check_circle_overlap(Vec2::ZERO, 10.0, Vec2::new(25.0, 0.0), 10.0));
  }

  #[test]
  fn overlap_concentric_circles() {
    assert!(check_circle_overlap(
      Vec2::new(5.0, 5.0),
      10.0,
      Vec2::new(5.0, 5.0),
      3.0
    ));
  }

  #[test]
  fn overlap_barely_touching() {
    // Exactly at boundary — distance = combined radius
    // distance_squared < combined² is false when equal
    assert!(!check_circle_overlap(Vec2::ZERO, 5.0, Vec2::new(10.0, 0.0), 5.0));
  }

  #[test]
  fn boundary_center_is_safe() {
    let bounds = ArenaBounds {
      min: Vec2::new(-100.0, -100.0),
      max: Vec2::new(100.0, 100.0),
    };
    assert!(!check_boundary(Vec2::ZERO, 10.0, &bounds));
  }

  #[test]
  fn boundary_left_edge_out() {
    let bounds = ArenaBounds {
      min: Vec2::new(-100.0, -100.0),
      max: Vec2::new(100.0, 100.0),
    };
    assert!(check_boundary(Vec2::new(-95.0, 0.0), 10.0, &bounds));
  }

  #[test]
  fn boundary_right_edge_out() {
    let bounds = ArenaBounds {
      min: Vec2::new(-100.0, -100.0),
      max: Vec2::new(100.0, 100.0),
    };
    assert!(check_boundary(Vec2::new(95.0, 0.0), 10.0, &bounds));
  }

  #[test]
  fn boundary_top_edge_out() {
    let bounds = ArenaBounds {
      min: Vec2::new(-100.0, -100.0),
      max: Vec2::new(100.0, 100.0),
    };
    assert!(check_boundary(Vec2::new(0.0, 95.0), 10.0, &bounds));
  }

  #[test]
  fn boundary_bottom_edge_out() {
    let bounds = ArenaBounds {
      min: Vec2::new(-100.0, -100.0),
      max: Vec2::new(100.0, 100.0),
    };
    assert!(check_boundary(Vec2::new(0.0, -95.0), 10.0, &bounds));
  }

  #[test]
  fn boundary_just_inside() {
    let bounds = ArenaBounds {
      min: Vec2::new(-100.0, -100.0),
      max: Vec2::new(100.0, 100.0),
    };
    // pos.x + radius = 90 + 10 = 100, which is NOT > 100
    assert!(!check_boundary(Vec2::new(90.0, 0.0), 10.0, &bounds));
  }
}
