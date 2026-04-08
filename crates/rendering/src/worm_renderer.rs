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

pub(crate) struct WormRendererPlugin;

impl Plugin for WormRendererPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<WormSpriteRegistry>()
      .init_resource::<WormNameRegistry>()
      .add_systems(PreStartup, load_worm_textures)
      .add_systems(Update, (sync_worm_sprites, sync_worm_names));
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
  theme: Res<GruvboxTheme>,
) {
  let Some(world) = world else { return };
  let textures = match textures {
    Some(t) => t,
    None => return,
  };

  let mut live_keys: HashMap<(WormId, usize), WormVisual> = HashMap::new();

  // Player
  let player = world.player();
  if player.is_alive() {
    for (i, seg) in player.segments().iter().enumerate() {
      let z = if i == 0 { 5.0 } else { 4.0 };
      let size = if i == 0 { seg.radius() * 2.8 } else { seg.radius() * 2.2 };
      let texture = if i == 0 {
        textures.player_head.clone()
      } else {
        textures.player_segment.clone()
      };
      live_keys.insert(
        (player.id(), i),
        WormVisual {
          pos: seg.position().extend(z),
          size,
          texture,
        },
      );
    }
  }

  // AI worms
  for (idx, (worm, _)) in world.ai_worms().iter().enumerate() {
    if !worm.is_alive() {
      continue;
    }
    let tex_idx = idx % textures.heads.len();
    for (i, seg) in worm.segments().iter().enumerate() {
      let z = if i == 0 { 3.0 } else { 2.0 };
      let size = if i == 0 { seg.radius() * 2.8 } else { seg.radius() * 2.2 };
      let texture = if i == 0 {
        textures.heads[tex_idx].clone()
      } else {
        textures.segments[tex_idx].clone()
      };
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
        // Update size — worms grow wider over time
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

  // Keep theme reference alive to suppress unused warning
  let _ = &theme;
}

fn sync_worm_names(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  mut name_reg: ResMut<WormNameRegistry>,
  mut labels: Query<(&mut Transform, &mut Text2d, &mut TextColor), With<WormNameLabel>>,
  theme: Res<GruvboxTheme>,
  fonts: Res<crate::fonts::GameFonts>,
) {
  let Some(world) = world else { return };

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

  // Map worm_id → rank (1-based), only for top 10
  let mut rank_map: HashMap<WormId, usize> = HashMap::new();
  for (i, (id, _)) in ranked.iter().enumerate() {
    if i < 10 {
      rank_map.insert(*id, i + 1);
    }
  }

  // Collect live worm IDs and their display info
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

/// Format worm label: "#N name" for top 10, just "name" otherwise.
fn format_worm_label(name: &str, rank: Option<&usize>) -> String {
  match rank {
    Some(r) => format!("#{} {}", r, name),
    None => name.to_string(),
  }
}
