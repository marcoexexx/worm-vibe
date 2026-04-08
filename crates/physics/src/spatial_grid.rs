use glam::Vec2;
use std::collections::HashMap;

type GridCell<T> = Vec<(T, Vec2, f32)>;

/// Broad-phase spatial hash for fast neighbor lookups.
pub struct SpatialHashGrid<T: Copy + Eq + std::hash::Hash> {
  cell_size: f32,
  cells: HashMap<(i32, i32), GridCell<T>>,
}

impl<T: Copy + Eq + std::hash::Hash> SpatialHashGrid<T> {
  pub fn new(cell_size: f32) -> Self {
    Self {
      cell_size,
      cells: HashMap::new(),
    }
  }

  pub fn clear(&mut self) {
    self.cells.clear();
  }

  pub fn insert(&mut self, id: T, pos: Vec2, radius: f32) {
    let key = self.cell_key(pos);
    self.cells.entry(key).or_default().push((id, pos, radius));
  }

  /// Query all entities in the same or adjacent cells.
  pub fn query_neighbors(&self, pos: Vec2) -> Vec<(T, Vec2, f32)> {
    let (cx, cy) = self.cell_key(pos);
    let mut results = Vec::new();

    for dx in -1..=1 {
      for dy in -1..=1 {
        if let Some(bucket) = self.cells.get(&(cx + dx, cy + dy)) {
          results.extend_from_slice(bucket);
        }
      }
    }

    results
  }

  fn cell_key(&self, pos: Vec2) -> (i32, i32) {
    (
      (pos.x / self.cell_size).floor() as i32,
      (pos.y / self.cell_size).floor() as i32,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn insert_and_query_same_cell() {
    let mut grid = SpatialHashGrid::new(100.0);
    grid.insert(1u32, Vec2::new(50.0, 50.0), 5.0);
    let results = grid.query_neighbors(Vec2::new(60.0, 60.0));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, 1);
  }

  #[test]
  fn query_empty_grid() {
    let grid = SpatialHashGrid::<u32>::new(100.0);
    let results = grid.query_neighbors(Vec2::ZERO);
    assert!(results.is_empty());
  }

  #[test]
  fn query_adjacent_cell() {
    let mut grid = SpatialHashGrid::new(50.0);
    // Insert in cell (0,0)
    grid.insert(1u32, Vec2::new(10.0, 10.0), 5.0);
    // Query from cell (1,0) — adjacent, should find it
    let results = grid.query_neighbors(Vec2::new(55.0, 10.0));
    assert_eq!(results.len(), 1);
  }

  #[test]
  fn query_far_cell_misses() {
    let mut grid = SpatialHashGrid::new(50.0);
    grid.insert(1u32, Vec2::new(10.0, 10.0), 5.0);
    // Query from cell (3,3) — not adjacent to (0,0)
    let results = grid.query_neighbors(Vec2::new(160.0, 160.0));
    assert!(results.is_empty());
  }

  #[test]
  fn multiple_entities_same_cell() {
    let mut grid = SpatialHashGrid::new(100.0);
    grid.insert(1u32, Vec2::new(10.0, 10.0), 5.0);
    grid.insert(2u32, Vec2::new(20.0, 20.0), 5.0);
    grid.insert(3u32, Vec2::new(30.0, 30.0), 5.0);
    let results = grid.query_neighbors(Vec2::new(15.0, 15.0));
    assert_eq!(results.len(), 3);
  }

  #[test]
  fn clear_empties_grid() {
    let mut grid = SpatialHashGrid::new(100.0);
    grid.insert(1u32, Vec2::new(10.0, 10.0), 5.0);
    grid.clear();
    let results = grid.query_neighbors(Vec2::new(10.0, 10.0));
    assert!(results.is_empty());
  }

  #[test]
  fn negative_coordinates_work() {
    let mut grid = SpatialHashGrid::new(100.0);
    grid.insert(1u32, Vec2::new(-50.0, -50.0), 5.0);
    let results = grid.query_neighbors(Vec2::new(-40.0, -40.0));
    assert_eq!(results.len(), 1);
  }

  #[test]
  fn query_returns_position_and_radius() {
    let mut grid = SpatialHashGrid::new(100.0);
    let pos = Vec2::new(25.0, 75.0);
    grid.insert(42u32, pos, 12.0);
    let results = grid.query_neighbors(pos);
    assert_eq!(results[0].0, 42);
    assert_eq!(results[0].1, pos);
    assert!((results[0].2 - 12.0).abs() < f32::EPSILON);
  }

  #[test]
  fn diagonal_adjacent_cells_found() {
    let mut grid = SpatialHashGrid::new(50.0);
    // cell (0,0)
    grid.insert(1u32, Vec2::new(45.0, 45.0), 5.0);
    // Query from cell (1,1) — diagonal neighbor
    let results = grid.query_neighbors(Vec2::new(55.0, 55.0));
    assert_eq!(results.len(), 1);
  }
}
