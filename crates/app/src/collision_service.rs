use domain::{ArenaBounds, CollisionConfig, Food, Worm};
use physics::{check_boundary, check_circle_overlap, CollisionEntity, CollisionResult, SpatialHashGrid};

/// Run collision detection for all worms against food, boundaries, and each other.
/// Uses swept collision — samples intermediate points along the head's movement
/// path to prevent fast worms from tunneling through obstacles.
pub(crate) fn detect_collisions(
  player: &Worm,
  ai_worms: &[Worm],
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

  let all_worms: Vec<&Worm> = std::iter::once(player).chain(ai_worms.iter()).collect();

  // Populate grid with worm segments (skip head at index 0).
  // Use shrunk radius for worm body hitboxes so collisions feel fair.
  let shrink = cfg.worm_hitbox_shrink;
  for worm in &all_worms {
    if !worm.is_alive() {
      continue;
    }
    for (si, seg) in worm.segments().iter().enumerate() {
      if si == 0 {
        continue;
      }
      grid.insert(
        CollisionEntity::WormSegment(worm.id(), si),
        seg.position(),
        seg.radius() * shrink,
      );
    }
  }

  // Check each worm's head — with swept collision
  for worm in &all_worms {
    if !worm.is_alive() {
      continue;
    }

    let head_pos = worm.head_position();
    let prev_pos = worm.prev_head_position();
    let head_r = worm.head_radius();
    let head_r_shrunk = head_r * shrink; // smaller hitbox for worm-vs-worm

    // Boundary check (uses full radius — walls are hard)
    if check_boundary(head_pos, head_r, bounds) {
      results.push(CollisionResult::WormHitBoundary { worm_id: worm.id() });
      continue;
    }

    // Determine how many sample points along the path
    let travel_dist = head_pos.distance(prev_pos);
    let step_size = head_r * 1.5; // sample every ~1.5 radii
    let num_steps = if travel_dist > step_size {
      ((travel_dist / step_size).ceil() as usize).min(10)
    } else {
      1
    };

    let mut found_collision = false;

    for step in 0..num_steps {
      if found_collision {
        break;
      }

      // Interpolate position along the path
      let t = if num_steps == 1 {
        1.0
      } else {
        step as f32 / (num_steps - 1) as f32
      };
      let sample_pos = prev_pos.lerp(head_pos, t);

      for (entity, pos, radius) in grid.query_neighbors(sample_pos) {
        match entity {
          CollisionEntity::Food(idx) => {
            // Food uses full radius for generous pickup
            if !check_circle_overlap(sample_pos, head_r, pos, radius) {
              continue;
            }
            results.push(CollisionResult::WormAteFood {
              worm_id: worm.id(),
              food_index: idx,
            });
          }
          CollisionEntity::WormSegment(owner_id, seg_idx) => {
            // Worm-vs-worm uses shrunk hitbox
            if !check_circle_overlap(sample_pos, head_r_shrunk, pos, radius) {
              continue;
            }
            // No self-collision
            if owner_id == worm.id() {
              continue;
            }
            // Grace zone near other worm's head
            if seg_idx < cfg.other_segment_skip {
              continue;
            }
            // Noclip cheat
            if worm.has_noclip() {
              continue;
            }

            results.push(CollisionResult::WormHitWorm {
              victim: worm.id(),
              killer: owner_id,
            });
            found_collision = true;
            break;
          }
          CollisionEntity::WormHead(_) => {}
        }
      }
    }
  }

  results
}
