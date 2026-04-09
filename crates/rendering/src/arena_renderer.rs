use app::GameWorld;
use bevy::prelude::*;

use crate::theme::GruvboxTheme;

const TILE_SIZE: f32 = 400.0;
const BOUNDARY_THICKNESS: f32 = 4.0;

#[derive(Component)]
struct BoardTile;

#[derive(Component)]
struct ArenaBoundary;

/// Tracks whether the arena has been spawned for the current game world.
#[derive(Resource, Default)]
struct ArenaSpawned {
  spawned_for_width: f32,
}

pub(crate) struct ArenaRendererPlugin;

impl Plugin for ArenaRendererPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<ArenaSpawned>().add_systems(Update, sync_arena);
  }
}

/// Spawn/respawn arena visuals when the game world changes size.
fn sync_arena(
  mut commands: Commands,
  theme: Res<GruvboxTheme>,
  world: Option<Res<GameWorld>>,
  asset_server: Res<AssetServer>,
  mut spawned: ResMut<ArenaSpawned>,
  tile_q: Query<Entity, With<BoardTile>>,
  boundary_q: Query<Entity, With<ArenaBoundary>>,
) {
  let Some(world) = world else { return };
  let arena = world.config().arena();
  let half = arena.half_extents();

  // Only re-spawn if arena size changed
  if (spawned.spawned_for_width - arena.width()).abs() < 1.0 {
    return;
  }

  // Despawn old
  for entity in tile_q.iter() {
    commands.entity(entity).despawn();
  }
  for entity in boundary_q.iter() {
    commands.entity(entity).despawn();
  }

  spawned.spawned_for_width = arena.width();

  // Board background — tile the board_bg.png across the arena
  let bg_texture: Handle<Image> = asset_server.load("textures/board_bg.png");
  let extent = half.x.max(half.y);

  let mut tx = -extent;
  while tx < extent {
    let mut ty = -extent;
    while ty < extent {
      commands.spawn((
        Sprite {
          image: bg_texture.clone(),
          custom_size: Some(Vec2::splat(TILE_SIZE)),
          ..default()
        },
        Transform::from_xyz(tx + TILE_SIZE / 2.0, ty + TILE_SIZE / 2.0, -10.0),
        BoardTile,
      ));
      ty += TILE_SIZE;
    }
    tx += TILE_SIZE;
  }

  // Boundary lines — exactly matching the collision kill zone
  let color = theme.red.with_alpha(0.6);
  let hx = half.x;
  let hy = half.y;
  let t = BOUNDARY_THICKNESS;
  let full_w = hx * 2.0 + t;
  let full_h = hy * 2.0 + t;

  let sides = [
    (0.0, hy, full_w, t),  // top
    (0.0, -hy, full_w, t), // bottom
    (-hx, 0.0, t, full_h), // left
    (hx, 0.0, t, full_h),  // right
  ];

  for (x, y, w, h) in sides {
    commands.spawn((
      Sprite {
        color,
        custom_size: Some(Vec2::new(w, h)),
        ..default()
      },
      Transform::from_xyz(x, y, -5.0),
      ArenaBoundary,
    ));
  }
}
