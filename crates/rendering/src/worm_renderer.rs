use app::GameWorld;
use bevy::prelude::*;
use bevy::utils::HashSet;
use domain::WormId;
use glam::Vec2;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

// ============================================================================
// Components
// ============================================================================

#[derive(Component)]
pub(crate) struct WormSegmentSprite {
  worm_id: WormId,
  segment_index: usize,
}

#[derive(Component)]
struct WormNameLabel(WormId);

#[derive(Component)]
struct KingCrownSprite;

// ============================================================================
// Resources
// ============================================================================

#[derive(Resource)]
struct WormTextures {
  heads: Vec<Handle<Image>>,
  segments: Vec<Handle<Image>>,
  player_head: Handle<Image>,
  player_segment: Handle<Image>,
}

#[derive(Resource, Default)]
struct SpritePool {
  /// All segment sprite entities, keyed by (worm_id, seg_index).
  /// Uses a flat Vec for fast iteration — no HashMap overhead.
  entries: Vec<PoolEntry>,
}

struct PoolEntry {
  worm_id: WormId,
  seg_index: usize,
  entity: Entity,
}

#[derive(Resource, Default)]
struct NamePool {
  entries: Vec<(WormId, Entity)>,
}

/// Frustum cull: segments beyond this px from camera center are hidden.
const CULL_RADIUS: f32 = 1200.0;
const CULL_RADIUS_SQ: f32 = CULL_RADIUS * CULL_RADIUS;
/// Worms whose HEAD is beyond this distance are fully skipped.
const WORM_CULL_RADIUS: f32 = 2000.0;
const WORM_CULL_SQ: f32 = WORM_CULL_RADIUS * WORM_CULL_RADIUS;

// ============================================================================
// Plugin
// ============================================================================

pub(crate) struct WormRendererPlugin;

impl Plugin for WormRendererPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<SpritePool>()
      .init_resource::<NamePool>()
      .add_systems(PreStartup, load_worm_textures)
      .add_systems(Update, (sync_worm_sprites, sync_worm_names, sync_king_crown));
  }
}

fn load_worm_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
  let color_names = ["red", "yellow", "blue", "purple", "orange", "aqua"];
  let heads: Vec<_> = color_names
    .iter()
    .map(|n| asset_server.load(format!("textures/worm_head_{n}.png")))
    .collect();
  let segments: Vec<_> = color_names
    .iter()
    .map(|n| asset_server.load(format!("textures/worm_segment_{n}.png")))
    .collect();
  commands.insert_resource(WormTextures {
    heads,
    segments,
    player_head: asset_server.load("textures/worm_head_green.png"),
    player_segment: asset_server.load("textures/worm_segment_green.png"),
  });
}

// ============================================================================
// Sprite sync — zero HashMap, direct entity updates
// ============================================================================

fn sync_worm_sprites(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  mut pool: ResMut<SpritePool>,
  mut sprite_q: Query<(&mut Transform, &mut Sprite, &mut Visibility)>,
  textures: Option<Res<WormTextures>>,
  camera_q: Query<&Transform, (With<Camera2d>, Without<Sprite>)>,
) {
  let Some(world) = world else { return };
  let Some(textures) = textures else { return };

  let cam_pos = camera_q
    .get_single()
    .map(|t| t.translation.truncate())
    .unwrap_or_default();

  // Track which (worm_id, seg_index) are still alive
  let mut alive_set: HashSet<(WormId, usize)> = HashSet::new();

  // --- Update or spawn for player ---
  let player = world.player();
  if player.is_alive() {
    update_worm_segments(
      &mut commands,
      &mut pool,
      &mut sprite_q,
      player,
      &textures.player_head,
      &textures.player_segment,
      true,
      cam_pos,
      &mut alive_set,
    );
  }

  // --- Update or spawn for AI worms ---
  for (idx, (worm, _)) in world.ai_worms().iter().enumerate() {
    if !worm.is_alive() {
      continue;
    }
    // Whole-worm cull by head distance
    if worm.head_position().distance_squared(cam_pos) > WORM_CULL_SQ {
      continue;
    }
    let tex_idx = idx % textures.heads.len();
    update_worm_segments(
      &mut commands,
      &mut pool,
      &mut sprite_q,
      worm,
      &textures.heads[tex_idx],
      &textures.segments[tex_idx],
      false,
      cam_pos,
      &mut alive_set,
    );
  }

  // --- Despawn dead entries ---
  pool.entries.retain(|entry| {
    let key = (entry.worm_id, entry.seg_index);
    if alive_set.contains(&key) {
      true
    } else {
      commands.entity(entry.entity).despawn();
      false
    }
  });
}

/// Update or spawn sprites for a single worm's segments.
fn update_worm_segments(
  commands: &mut Commands,
  pool: &mut SpritePool,
  sprite_q: &mut Query<(&mut Transform, &mut Sprite, &mut Visibility)>,
  worm: &domain::Worm,
  head_tex: &Handle<Image>,
  seg_tex: &Handle<Image>,
  is_player: bool,
  cam_pos: Vec2,
  alive_set: &mut HashSet<(WormId, usize)>,
) {
  let worm_id = worm.id();
  let z_head: f32 = if is_player { 5.0 } else { 3.0 };
  let z_seg: f32 = if is_player { 4.0 } else { 2.0 };

  for (i, seg) in worm.segments().iter().enumerate() {
    let seg_pos = seg.position();

    // Per-segment frustum cull (except head — always show if worm is visible)
    if i > 0 && seg_pos.distance_squared(cam_pos) > CULL_RADIUS_SQ {
      continue;
    }

    alive_set.insert((worm_id, i));

    let z = if i == 0 { z_head } else { z_seg };
    let size = if i == 0 {
      seg.radius() * 2.8
    } else {
      seg.radius() * 2.2
    };
    let pos3 = seg_pos.extend(z);

    // Try to find existing entity in pool (linear scan — fast for typical counts)
    if let Some(entry) = pool
      .entries
      .iter()
      .find(|e| e.worm_id == worm_id && e.seg_index == i)
    {
      if let Ok((mut transform, mut sprite, mut vis)) = sprite_q.get_mut(entry.entity) {
        transform.translation = pos3;
        sprite.custom_size = Some(Vec2::splat(size));
        *vis = Visibility::Inherited;
      }
    } else {
      // Spawn new
      let texture = if i == 0 { head_tex.clone() } else { seg_tex.clone() };
      let entity = commands
        .spawn((
          Sprite {
            image: texture,
            custom_size: Some(Vec2::splat(size)),
            ..default()
          },
          Transform::from_translation(pos3),
          WormSegmentSprite {
            worm_id,
            segment_index: i,
          },
        ))
        .id();
      pool.entries.push(PoolEntry {
        worm_id,
        seg_index: i,
        entity,
      });
    }
  }
}

// ============================================================================
// Name labels — simplified, no HashMap
// ============================================================================

fn sync_worm_names(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  mut pool: ResMut<NamePool>,
  mut labels: Query<(&mut Transform, &mut Text2d, &mut TextColor, &mut Visibility), With<WormNameLabel>>,
  theme: Res<GruvboxTheme>,
  fonts: Res<crate::fonts::GameFonts>,
  camera_q: Query<&Transform, (With<Camera2d>, Without<WormNameLabel>)>,
) {
  let Some(world) = world else { return };

  let cam_pos = camera_q
    .get_single()
    .map(|t| t.translation.truncate())
    .unwrap_or_default();

  // Build rank map (only top 10)
  let mut ranked: Vec<(WormId, usize)> = Vec::new();
  if world.player().is_alive() {
    ranked.push((world.player().id(), world.player().length()));
  }
  for (worm, _) in world.ai_worms() {
    if worm.is_alive() {
      ranked.push((worm.id(), worm.length()));
    }
  }
  ranked.sort_unstable_by(|a, b| b.1.cmp(&a.1));

  let alive_ids: HashSet<WormId> = ranked.iter().map(|(id, _)| *id).collect();

  // Inline rank lookup (avoids HashMap)
  let rank_of = |id: WormId| -> Option<usize> {
    ranked.iter().position(|(rid, _)| *rid == id).and_then(|i| if i < 10 { Some(i + 1) } else { None })
  };

  // Update or spawn labels for visible worms
  let mut seen: HashSet<WormId> = HashSet::new();

  // Player always visible
  if world.player().is_alive() {
    let id = world.player().id();
    let label = format_worm_label(world.player().name(), rank_of(id));
    upsert_label(
      &mut commands,
      &mut pool,
      &mut labels,
      &fonts,
      id,
      world.player().head_position(),
      &label,
      theme.green,
    );
    seen.insert(id);
  }

  for (idx, (worm, _)) in world.ai_worms().iter().enumerate() {
    if !worm.is_alive() {
      continue;
    }
    if worm.head_position().distance_squared(cam_pos) > WORM_CULL_SQ {
      continue;
    }
    let id = worm.id();
    let label = format_worm_label(worm.name(), rank_of(id));
    upsert_label(
      &mut commands,
      &mut pool,
      &mut labels,
      &fonts,
      id,
      worm.head_position(),
      &label,
      theme.worm_color(idx),
    );
    seen.insert(id);
  }

  // Hide or despawn labels for dead/culled worms
  pool.entries.retain(|(id, entity)| {
    if seen.contains(id) && alive_ids.contains(id) {
      true
    } else {
      commands.entity(*entity).despawn();
      false
    }
  });
}

fn upsert_label(
  commands: &mut Commands,
  pool: &mut NamePool,
  labels: &mut Query<(&mut Transform, &mut Text2d, &mut TextColor, &mut Visibility), With<WormNameLabel>>,
  fonts: &GameFonts,
  id: WormId,
  head_pos: Vec2,
  text: &str,
  color: Color,
) {
  let label_pos = Vec3::new(head_pos.x, head_pos.y + 24.0, 20.0);

  if let Some((_, entity)) = pool.entries.iter().find(|(eid, _)| *eid == id) {
    if let Ok((mut transform, mut t, mut tc, mut vis)) = labels.get_mut(*entity) {
      transform.translation = label_pos;
      **t = text.to_string();
      tc.0 = color;
      *vis = Visibility::Inherited;
    }
  } else {
    let entity = commands
      .spawn((
        Text2d::new(text.to_string()),
        TextFont {
          font: fonts.bold.clone(),
          font_size: 12.0,
          ..default()
        },
        TextColor(color),
        Transform::from_translation(label_pos),
        WormNameLabel(id),
      ))
      .id();
    pool.entries.push((id, entity));
  }
}

fn format_worm_label(name: &str, rank: Option<usize>) -> String {
  match rank {
    Some(r) => format!("#{} {}", r, name),
    None => name.to_string(),
  }
}

// ============================================================================
// King crown sprite
// ============================================================================

fn sync_king_crown(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  asset_server: Res<AssetServer>,
  mut crown_q: Query<(&mut Transform, &mut Visibility), With<KingCrownSprite>>,
) {
  let Some(world) = world else {
    for (_, mut vis) in &mut crown_q {
      *vis = Visibility::Hidden;
    }
    return;
  };

  // Find king
  let mut king_pos: Option<Vec2> = None;
  let mut king_len = 0usize;
  if world.player().is_alive() {
    king_len = world.player().length();
    king_pos = Some(world.player().head_position());
  }
  for (worm, _) in world.ai_worms() {
    if worm.is_alive() && worm.length() > king_len {
      king_len = worm.length();
      king_pos = Some(worm.head_position());
    }
  }

  let Some(pos) = king_pos else {
    for (_, mut vis) in &mut crown_q {
      *vis = Visibility::Hidden;
    }
    return;
  };

  let crown_pos = Vec3::new(pos.x, pos.y + 40.0, 25.0);

  if let Ok((mut transform, mut vis)) = crown_q.get_single_mut() {
    transform.translation = crown_pos;
    *vis = Visibility::Inherited;
  } else {
    commands.spawn((
      Sprite {
        image: asset_server.load("textures/crown.png"),
        custom_size: Some(Vec2::new(24.0, 24.0)),
        ..default()
      },
      Transform::from_translation(crown_pos),
      KingCrownSprite,
    ));
  }
}
