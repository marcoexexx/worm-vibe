mod collision;
mod movement;
mod spatial_grid;

pub use collision::{check_boundary, check_circle_overlap, CollisionEntity, CollisionResult};
pub use movement::{follow_segment, move_head, smooth_turn};
pub use spatial_grid::SpatialHashGrid;
