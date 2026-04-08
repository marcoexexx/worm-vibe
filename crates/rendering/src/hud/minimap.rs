use app::GameWorld;
use bevy::prelude::*;
use glam::Vec2;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

const MINIMAP_SIZE: f32 = 150.0;
const MINIMAP_MARGIN: f32 = 12.0;
const DOT_SIZE: f32 = 4.0;
const PLAYER_DOT_SIZE: f32 = 6.0;
/// Max dots in the minimap pool (player + AI + sampled food).
const MAX_DOTS: usize = 48;

#[derive(Component)]
struct MinimapRoot;

/// Pooled minimap dot — reused each frame instead of despawn/respawn.
#[derive(Component)]
struct MinimapDot {
  index: usize,
}

/// Arrow on screen pointing toward the #1 worm.
#[derive(Component)]
struct TopWormArrow;

pub(crate) struct MinimapPlugin;

impl Plugin for MinimapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_minimap_frame)
      .add_systems(Update, (update_minimap, update_top_arrow));
  }
}

fn spawn_minimap_frame(mut commands: Commands, theme: Res<GruvboxTheme>, fonts: Res<GameFonts>) {
  // Minimap container
  commands
    .spawn((
      Node {
        position_type: PositionType::Absolute,
        top: Val::Px(MINIMAP_MARGIN),
        right: Val::Px(MINIMAP_MARGIN),
        width: Val::Px(MINIMAP_SIZE),
        height: Val::Px(MINIMAP_SIZE),
        border: UiRect::all(Val::Px(1.0)),
        overflow: Overflow::clip(),
        ..default()
      },
      BackgroundColor(theme.bg_hard.with_alpha(0.85)),
      BorderColor(theme.gray.with_alpha(0.4)),
      MinimapRoot,
    ))
    .with_children(|parent| {
      // "MAP" label
      parent.spawn((
        Text::new("MAP"),
        TextFont {
          font: fonts.regular.clone(),
          font_size: 9.0,
          ..default()
        },
        TextColor(theme.gray),
        Node {
          position_type: PositionType::Absolute,
          bottom: Val::Px(2.0),
          right: Val::Px(4.0),
          ..default()
        },
      ));

      // Pre-spawn dot pool (hidden by default)
      for i in 0..MAX_DOTS {
        parent.spawn((
          Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Px(DOT_SIZE),
            height: Val::Px(DOT_SIZE),
            ..default()
          },
          BackgroundColor(Color::NONE),
          Visibility::Hidden,
          MinimapDot { index: i },
        ));
      }
    });

  // Direction arrow
  commands.spawn((
    Text::new(""),
    TextFont {
      font: fonts.bold.clone(),
      font_size: 16.0,
      ..default()
    },
    TextColor(theme.red),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(40.0),
      right: Val::Px(MINIMAP_MARGIN + MINIMAP_SIZE + 8.0),
      ..default()
    },
    TopWormArrow,
  ));
}

fn update_minimap(
  world: Option<Res<GameWorld>>,
  theme: Res<GruvboxTheme>,
  mut dots: Query<(&MinimapDot, &mut Node, &mut BackgroundColor, &mut Visibility)>,
) {
  let Some(world) = world else {
    // Hide all dots
    for (_, _, _, mut vis) in &mut dots {
      *vis = Visibility::Hidden;
    }
    return;
  };

  let arena_half = world.config().arena().half_extents();

  let to_map = |world_pos: Vec2| -> (f32, f32) {
    let nx = ((world_pos.x / arena_half.x) * 0.5 + 0.5) * MINIMAP_SIZE;
    let ny = ((-world_pos.y / arena_half.y) * 0.5 + 0.5) * MINIMAP_SIZE;
    (nx.clamp(1.0, MINIMAP_SIZE - 2.0), ny.clamp(1.0, MINIMAP_SIZE - 2.0))
  };

  // Find #1 worm
  let mut top1_id = world.player().id();
  let mut top1_len = world.player().length();
  for (worm, _) in world.ai_worms() {
    if worm.is_alive() && worm.length() > top1_len {
      top1_id = worm.id();
      top1_len = worm.length();
    }
  }

  // Build dot data: (x, y, size, color)
  let mut dot_data: Vec<(f32, f32, f32, Color)> = Vec::with_capacity(MAX_DOTS);

  // Food dots (sample every Nth to stay within budget)
  let foods = world.foods();
  let food_step = (foods.len() / 20).max(1);
  for (i, food) in foods.iter().enumerate() {
    if i % food_step != 0 || dot_data.len() >= MAX_DOTS - 14 {
      break;
    }
    let (x, y) = to_map(food.position());
    dot_data.push((x, y, 2.0, theme.gray.with_alpha(0.3)));
  }

  // AI worm dots
  for (idx, (worm, _)) in world.ai_worms().iter().enumerate() {
    if !worm.is_alive() || dot_data.len() >= MAX_DOTS - 1 {
      continue;
    }
    let (x, y) = to_map(worm.head_position());
    let is_top1 = worm.id() == top1_id;
    let color = if is_top1 { theme.red } else { theme.worm_color(idx) };
    let size = if is_top1 { PLAYER_DOT_SIZE } else { DOT_SIZE };
    dot_data.push((x, y, size, color));
  }

  // Player dot (on top)
  if world.player().is_alive() {
    let (x, y) = to_map(world.player().head_position());
    dot_data.push((x, y, PLAYER_DOT_SIZE, theme.green));
  }

  // Apply to pooled dot entities
  for (dot, mut node, mut bg, mut vis) in &mut dots {
    if dot.index < dot_data.len() {
      let (x, y, size, color) = dot_data[dot.index];
      node.left = Val::Px(x - size / 2.0);
      node.top = Val::Px(y - size / 2.0);
      node.width = Val::Px(size);
      node.height = Val::Px(size);
      bg.0 = color;
      *vis = Visibility::Inherited;
    } else {
      *vis = Visibility::Hidden;
    }
  }
}

fn update_top_arrow(
  world: Option<Res<GameWorld>>,
  mut arrow_q: Query<(&mut Text, &mut Visibility), With<TopWormArrow>>,
) {
  let Ok((mut text, mut vis)) = arrow_q.get_single_mut() else {
    return;
  };

  let Some(world) = world else {
    *vis = Visibility::Hidden;
    return;
  };

  if !world.player().is_alive() {
    *vis = Visibility::Hidden;
    return;
  }

  let mut top1_id = world.player().id();
  let mut top1_len = world.player().length();
  let mut top1_pos = world.player().head_position();
  let mut top1_score = world.player().score();
  for (worm, _) in world.ai_worms() {
    if worm.is_alive() && worm.length() > top1_len {
      top1_id = worm.id();
      top1_len = worm.length();
      top1_pos = worm.head_position();
      top1_score = worm.score();
    }
  }

  if top1_id == world.player().id() {
    **text = "[K] YOU ARE #1".to_string();
    *vis = Visibility::Visible;
    return;
  }

  let player_pos = world.player().head_position();
  let delta = top1_pos - player_pos;
  let dist = delta.length();
  let arrow = direction_arrow(delta);

  **text = format!("[K] {} ({:.0}m) {}", arrow, dist / 10.0, top1_score);
  *vis = Visibility::Visible;
}

fn direction_arrow(delta: Vec2) -> &'static str {
  if delta.length_squared() < 1.0 {
    return "o";
  }
  let angle = delta.y.atan2(delta.x);
  let octant = ((angle + std::f32::consts::PI) / (std::f32::consts::PI / 4.0)) as usize % 8;
  match octant {
    0 => "\u{2190}",
    1 => "\u{2199}",
    2 => "\u{2193}",
    3 => "\u{2198}",
    4 => "\u{2192}",
    5 => "\u{2197}",
    6 => "\u{2191}",
    7 => "\u{2196}",
    _ => "o",
  }
}
