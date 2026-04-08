use app::GameWorld;
use bevy::prelude::*;
use bevy::utils::HashMap;
use domain::WormId;

use crate::theme::GruvboxTheme;

/// Marker linking a sprite entity to a specific worm segment.
#[derive(Component)]
#[allow(dead_code)]
pub(crate) struct WormSegmentSprite {
  worm_id: WormId,
  segment_index: usize,
}

/// Pre-loaded worm textures indexed by color name.
#[derive(Resource)]
struct WormTextures {
  heads: Vec<Handle<Image>>, // indexed same as theme.worm_color
  segments: Vec<Handle<Image>>,
  player_head: Handle<Image>,
  player_segment: Handle<Image>,
}

/// Tracks all spawned worm sprite entities for diff-based sync.
#[derive(Resource, Default)]
struct WormSpriteRegistry {
  entities: HashMap<(WormId, usize), Entity>,
}

#[derive(Component)]
#[allow(dead_code)]
struct WormNameLabel(WormId);

#[derive(Resource, Default)]
struct WormNameRegistry {
  entities: HashMap<WormId, Entity>,
}

/// World-space crown sprite floating above the king's head.
#[derive(Component)]
struct KingCrownSprite;

/// Culling radius — worms beyond this distance from camera are not rendered.
const CULL_RADIUS: f32 = 2000.0;

pub(crate) struct WormRendererPlugin;

impl Plugin for WormRendererPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<WormSpriteRegistry>()
      .init_resource::<WormNameRegistry>()
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

struct WormVisual {
  pos: Vec3,
  size: f32,
  texture: Handle<Image>,
}

fn sync_worm_sprites(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  mut registry: ResMut<WormSpriteRegistry>,
  mut sprite_q: Query<(&mut Transform, &mut Sprite)>,
  textures: Option<Res<WormTextures>>,
  camera_q: Query<&Transform, (With<Camera2d>, Without<Sprite>)>,
  theme: Res<GruvboxTheme>,
) {
  let Some(world) = world else { return };
  let textures = match textures {
    Some(t) => t,
    None => return,
  };

  // Get camera center for frustum culling
  let cam_pos = camera_q
    .get_single()
    .map(|t| t.translation.truncate())
    .unwrap_or_default();
  let cull_sq = CULL_RADIUS * CULL_RADIUS;

  let mut live_keys: HashMap<(WormId, usize), WormVisual> = HashMap::new();

  // Player — always render (never cull yourself)
  let player = world.player();
  if player.is_alive() {
    add_worm_visuals(
      &mut live_keys,
      player,
      &textures.player_head,
      &textures.player_segment,
      true,
    );
  }

  // AI worms — cull if head is far from camera
  for (idx, (worm, _)) in world.ai_worms().iter().enumerate() {
    if !worm.is_alive() {
      continue;
    }
    // Frustum cull: skip worms whose head is too far from camera
    if worm.head_position().distance_squared(cam_pos) > cull_sq {
      continue;
    }
    let tex_idx = idx % textures.heads.len();
    add_worm_visuals(
      &mut live_keys,
      worm,
      &textures.heads[tex_idx],
      &textures.segments[tex_idx],
      false,
    );
  }

  // Remove dead entities
  registry.entities.retain(|key, entity| {
    if live_keys.contains_key(key) {
      true
    } else {
      commands.entity(*entity).despawn();
      false
    }
  });

  // Update existing / spawn new
  for (key, visual) in &live_keys {
    if let Some(entity) = registry.entities.get(key) {
      if let Ok((mut transform, mut sprite)) = sprite_q.get_mut(*entity) {
        transform.translation = visual.pos;
        sprite.custom_size = Some(Vec2::splat(visual.size));
      }
    } else {
      let entity = commands
        .spawn((
          Sprite {
            image: visual.texture.clone(),
            custom_size: Some(Vec2::splat(visual.size)),
            ..default()
          },
          Transform::from_translation(visual.pos),
          WormSegmentSprite {
            worm_id: key.0,
            segment_index: key.1,
          },
        ))
        .id();
      registry.entities.insert(*key, entity);
    }
  }

  let _ = &theme;
}

/// Add all segment visuals for a single worm to the live_keys map.
fn add_worm_visuals(
  live_keys: &mut HashMap<(WormId, usize), WormVisual>,
  worm: &domain::Worm,
  head_tex: &Handle<Image>,
  seg_tex: &Handle<Image>,
  is_player: bool,
) {
  let z_head = if is_player { 5.0 } else { 3.0 };
  let z_seg = if is_player { 4.0 } else { 2.0 };

  for (i, seg) in worm.segments().iter().enumerate() {
    let z = if i == 0 { z_head } else { z_seg };
    let size = if i == 0 { seg.radius() * 2.8 } else { seg.radius() * 2.2 };
    let texture = if i == 0 { head_tex.clone() } else { seg_tex.clone() };
    live_keys.insert(
      (worm.id(), i),
      WormVisual {
        pos: seg.position().extend(z),
        size,
        texture,
      },
    );
  }
}

fn sync_worm_names(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  mut name_reg: ResMut<WormNameRegistry>,
  mut labels: Query<(&mut Transform, &mut Text2d, &mut TextColor), With<WormNameLabel>>,
  theme: Res<GruvboxTheme>,
  fonts: Res<crate::fonts::GameFonts>,
  camera_q: Query<&Transform, (With<Camera2d>, Without<WormNameLabel>)>,
) {
  let Some(world) = world else { return };

  let cam_pos = camera_q
    .get_single()
    .map(|t| t.translation.truncate())
    .unwrap_or_default();
  let cull_sq = CULL_RADIUS * CULL_RADIUS;

  // Build ranking: sort all alive worms by length descending
  let mut ranked: Vec<(WormId, usize)> = Vec::new();
  if world.player().is_alive() {
    ranked.push((world.player().id(), world.player().length()));
  }
  for (worm, _) in world.ai_worms() {
    if worm.is_alive() {
      ranked.push((worm.id(), worm.length()));
    }
  }
  ranked.sort_by(|a, b| b.1.cmp(&a.1));

  let mut rank_map: HashMap<WormId, usize> = HashMap::new();
  for (i, (id, _)) in ranked.iter().enumerate() {
    if i < 10 {
      rank_map.insert(*id, i + 1);
    }
  }

  // Collect visible worms
  let mut live: HashMap<WormId, (Vec2, String, Color)> = HashMap::new();

  let player = world.player();
  if player.is_alive() {
    let label = format_worm_label(player.name(), rank_map.get(&player.id()));
    live.insert(player.id(), (player.head_position(), label, theme.green));
  }

  for (idx, (worm, _)) in world.ai_worms().iter().enumerate() {
    if !worm.is_alive() {
      continue;
    }
    // Cull distant worm labels
    if worm.head_position().distance_squared(cam_pos) > cull_sq {
      continue;
    }
    let label = format_worm_label(worm.name(), rank_map.get(&worm.id()));
    let color = theme.worm_color(idx);
    live.insert(worm.id(), (worm.head_position(), label, color));
  }

  // Remove dead labels
  name_reg.entities.retain(|id, entity| {
    if live.contains_key(id) {
      true
    } else {
      commands.entity(*entity).despawn();
      false
    }
  });

  // Update existing / spawn new
  for (id, (pos, label, color)) in &live {
    let label_pos = Vec3::new(pos.x, pos.y + 24.0, 20.0);

    if let Some(entity) = name_reg.entities.get(id) {
      if let Ok((mut transform, mut text, mut text_color)) = labels.get_mut(*entity) {
        transform.translation = label_pos;
        **text = label.clone();
        text_color.0 = *color;
      }
    } else {
      let entity = commands
        .spawn((
          Text2d::new(label.clone()),
          TextFont {
            font: fonts.bold.clone(),
            font_size: 12.0,
            ..default()
          },
          TextColor(*color),
          Transform::from_translation(label_pos),
          WormNameLabel(*id),
        ))
        .id();
      name_reg.entities.insert(*id, entity);
    }
  }
}

/// Format worm label: "#N name" for top 10.
fn format_worm_label(name: &str, rank: Option<&usize>) -> String {
  match rank {
    Some(r) => format!("#{} {}", r, name),
    None => name.to_string(),
  }
}

/// Spawn or move a single crown sprite above the king worm's head.
fn sync_king_crown(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  asset_server: Res<AssetServer>,
  mut crown_q: Query<(Entity, &mut Transform, &mut Visibility), With<KingCrownSprite>>,
) {
  let Some(world) = world else {
    for (_, _, mut vis) in &mut crown_q {
      *vis = Visibility::Hidden;
    }
    return;
  };

  // Find king (longest alive worm)
  let mut king_pos: Option<glam::Vec2> = None;
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
    for (_, _, mut vis) in &mut crown_q {
      *vis = Visibility::Hidden;
    }
    return;
  };

  // Crown floats above the name label (which is at +24, crown at +40)
  let crown_pos = Vec3::new(pos.x, pos.y + 40.0, 25.0);

  if let Ok((_, mut transform, mut vis)) = crown_q.get_single_mut() {
    transform.translation = crown_pos;
    *vis = Visibility::Inherited;
  } else {
    // Spawn crown entity (once)
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
