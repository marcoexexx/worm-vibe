use ai::BasicAiBrain;
use domain::{ArenaBounds, CollisionConfig, Food, Worm};
use physics::{check_boundary, check_circle_overlap, CollisionEntity, CollisionResult, SpatialHashGrid};

/// Run collision detection for all worms against food, boundaries, and each other.
/// Accepts the `(Worm, Brain)` tuple slice directly — no clone needed.
pub(crate) fn detect_collisions(
  player: &Worm,
  ai_worms: &[(Worm, BasicAiBrain)],
  foods: &[Food],
  bounds: &ArenaBounds,
  cfg: &CollisionConfig,
) -> Vec<CollisionResult> {
  let mut results = Vec::new();
  let mut grid = SpatialHashGrid::new(cfg.grid_cell_size);

  // Populate grid with food
  for (i, food) in foods.iter().enumerate() {
    grid.insert(CollisionEntity::Food(i), food.position(), food.radius());
  }

  // Collect all worm refs (player + alive AI)
  let mut all_worms: Vec<&Worm> = Vec::with_capacity(ai_worms.len() + 1);
  all_worms.push(player);
  for (worm, _) in ai_worms {
    all_worms.push(worm);
  }

  // Populate grid with worm body segments (skip head).
  let shrink = cfg.worm_hitbox_shrink;
  for worm in &all_worms {
    if !worm.is_alive() {
      continue;
    }
    for (si, seg) in worm.segments().iter().enumerate().skip(1) {
      grid.insert(
        CollisionEntity::WormSegment(worm.id(), si),
        seg.position(),
        seg.radius() * shrink,
      );
    }
  }

  // Check each worm's head with swept collision
  for worm in &all_worms {
    if !worm.is_alive() {
      continue;
    }
    check_worm_head(worm, &grid, bounds, cfg, shrink, &mut results);
  }

  results
}

/// Swept collision check for a single worm's head against the spatial grid.
fn check_worm_head(
  worm: &Worm,
  grid: &SpatialHashGrid<CollisionEntity>,
  bounds: &ArenaBounds,
  cfg: &CollisionConfig,
  shrink: f32,
  results: &mut Vec<CollisionResult>,
) {
  let head_pos = worm.head_position();
  let prev_pos = worm.prev_head_position();
  let head_r = worm.head_radius();
  let head_r_shrunk = head_r * shrink;

  // Boundary check (full radius — walls are hard)
  if check_boundary(head_pos, head_r, bounds) {
    results.push(CollisionResult::WormHitBoundary { worm_id: worm.id() });
    return;
  }

  // Swept collision: sample points along the head's movement path
  let travel_dist = head_pos.distance(prev_pos);
  let step_size = head_r * 1.5;
  let num_steps = if travel_dist > step_size {
    ((travel_dist / step_size).ceil() as usize).min(10)
  } else {
    1
  };

  for step in 0..num_steps {
    let t = if num_steps == 1 {
      1.0
    } else {
      step as f32 / (num_steps - 1) as f32
    };
    let sample_pos = prev_pos.lerp(head_pos, t);

    if check_sample_point(worm, sample_pos, head_r, head_r_shrunk, grid, cfg, results) {
      return; // found a lethal collision, stop checking
    }
  }
}

/// Check one sample point against the spatial grid. Returns true if a lethal hit was found.
fn check_sample_point(
  worm: &Worm,
  sample_pos: glam::Vec2,
  head_r: f32,
  head_r_shrunk: f32,
  grid: &SpatialHashGrid<CollisionEntity>,
  cfg: &CollisionConfig,
  results: &mut Vec<CollisionResult>,
) -> bool {
  for (entity, pos, radius) in grid.query_neighbors(sample_pos) {
    match entity {
      CollisionEntity::Food(idx) => {
        if check_circle_overlap(sample_pos, head_r, pos, radius) {
          results.push(CollisionResult::WormAteFood {
            worm_id: worm.id(),
            food_index: idx,
          });
        }
      }
      CollisionEntity::WormSegment(owner_id, seg_idx) => {
        if !check_circle_overlap(sample_pos, head_r_shrunk, pos, radius) {
          continue;
        }
        if owner_id == worm.id() {
          continue; // no self-collision
        }
        if seg_idx < cfg.other_segment_skip {
          continue; // grace zone near other worm's head
        }
        if worm.has_noclip() {
          continue;
        }
        results.push(CollisionResult::WormHitWorm {
          victim: worm.id(),
          killer: owner_id,
        });
        return true; // lethal
      }
      CollisionEntity::WormHead(_) => {}
    }
  }
  false
}
