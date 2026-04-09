use app::GameWorld;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::theme::GruvboxTheme;

/// A speed-line particle that streaks past while the player boosts.
#[derive(Component)]
struct SpeedLine {
  velocity: Vec2,
  lifetime: f32,
  max_lifetime: f32,
}

/// HUD energy bar background.
#[derive(Component)]
struct EnergyBarBg;

/// HUD energy bar fill (shrinks as segments drain).
#[derive(Component)]
struct EnergyBarFill;

/// Tracks the segment count when boosting started, so we can show drain progress.
#[derive(Resource, Default)]
struct BoostState {
  was_boosting: bool,
  start_length: usize,
  /// Track elapsed time to detect new game sessions.
  last_elapsed: f32,
}

pub(crate) struct BoostEffectPlugin;

impl Plugin for BoostEffectPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<BoostState>()
      .add_systems(Startup, spawn_energy_bar)
      .add_systems(Update, (spawn_speed_lines, tick_speed_lines, update_energy_bar));
  }
}

// --- Energy bar (HUD) ---

const BAR_WIDTH: f32 = 160.0;
const BAR_HEIGHT: f32 = 8.0;

fn spawn_energy_bar(mut commands: Commands, theme: Res<GruvboxTheme>) {
  // Container (bottom-center)
  commands
    .spawn((
      Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(24.0),
        left: Val::Percent(50.0),
        margin: UiRect {
          left: Val::Px(-BAR_WIDTH / 2.0),
          ..default()
        },
        width: Val::Px(BAR_WIDTH),
        height: Val::Px(BAR_HEIGHT),
        ..default()
      },
      BackgroundColor(theme.bg_hard.with_alpha(0.6)),
      BorderRadius::all(Val::Px(4.0)),
      EnergyBarBg,
    ))
    .with_children(|parent| {
      parent.spawn((
        Node {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        BackgroundColor(theme.green),
        BorderRadius::all(Val::Px(4.0)),
        EnergyBarFill,
      ));
    });
}

fn update_energy_bar(
  world: Option<Res<GameWorld>>,
  mut state: ResMut<BoostState>,
  mut bg_q: Query<&mut Visibility, With<EnergyBarBg>>,
  mut fill_q: Query<(&mut Node, &mut BackgroundColor), With<EnergyBarFill>>,
  theme: Res<GruvboxTheme>,
) {
  let Some(world) = world else {
    if let Ok(mut vis) = bg_q.get_single_mut() {
      *vis = Visibility::Hidden;
    }
    return;
  };

  let player = world.player();

  // Reset state when a new game starts (elapsed resets to near-zero)
  if world.elapsed() < state.last_elapsed {
    *state = BoostState::default();
  }
  state.last_elapsed = world.elapsed();

  let boosting = player.is_boosting();

  // Track when boost starts to snapshot the starting length
  if boosting && !state.was_boosting {
    state.start_length = player.length();
  }
  state.was_boosting = boosting;

  // Show bar only when player is alive and has enough segments to boost
  let can_boost = player.is_alive() && player.length() > 5;
  if let Ok(mut vis) = bg_q.get_single_mut() {
    *vis = if can_boost {
      Visibility::Inherited
    } else {
      Visibility::Hidden
    };
  }

  if !can_boost {
    return;
  }

  // Fill percentage: how many segments remain vs. start
  let min_seg = 5usize; // min_segments_for_boost
  let effective_start = state.start_length.max(player.length());
  let ratio = if effective_start <= min_seg {
    0.0
  } else {
    let current = player.length().saturating_sub(min_seg) as f32;
    let max = effective_start.saturating_sub(min_seg) as f32;
    if max > 0.0 {
      current / max
    } else {
      1.0
    }
  };

  if let Ok((mut node, mut bg)) = fill_q.get_single_mut() {
    node.width = Val::Percent(ratio * 100.0);
    // Color: green → yellow → red as energy drains
    bg.0 = if ratio > 0.5 {
      theme.green
    } else if ratio > 0.25 {
      theme.yellow
    } else {
      theme.red
    };
  }
}

// --- Speed lines (world-space) ---

fn spawn_speed_lines(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  theme: Res<GruvboxTheme>,
  time: Res<Time>,
  mut rng: Local<Option<StdRng>>,
) {
  let Some(world) = world else { return };
  let player = world.player();

  if !player.is_alive() || !player.is_boosting() {
    return;
  }

  let rng = rng.get_or_insert_with(StdRng::from_entropy);

  let dt = time.delta_secs();
  let head = player.head_position();
  let heading = player.heading();

  // Hoist trig outside the loop — constant for all particles this frame
  let perp = Vec2::new(-heading.sin(), heading.cos());
  let forward = Vec2::new(heading.cos(), heading.sin());

  // Spawn ~30 lines/sec
  let count = (30.0 * dt).ceil() as usize;

  for _ in 0..count {
    // Spawn behind and to the side of the player
    let side_offset = rng.gen_range(-60.0..60.0);
    let back_offset = rng.gen_range(-10.0..40.0);

    let pos = head + perp * side_offset - forward * back_offset;

    // Lines streak backward (opposite heading)
    let speed = rng.gen_range(200.0..400.0);
    let velocity = -forward * speed;

    let lifetime = rng.gen_range(0.15..0.3);

    // Thin stretched sprite
    let width = rng.gen_range(12.0..30.0);
    let height = rng.gen_range(1.5..3.0);

    commands.spawn((
      Sprite {
        color: theme.fg.with_alpha(0.3),
        custom_size: Some(Vec2::new(width, height)),
        ..default()
      },
      Transform {
        translation: pos.extend(6.0),
        rotation: Quat::from_rotation_z(heading),
        ..default()
      },
      SpeedLine {
        velocity,
        lifetime,
        max_lifetime: lifetime,
      },
    ));
  }
}

fn tick_speed_lines(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Transform, &mut SpeedLine, &mut Sprite)>,
  time: Res<Time>,
) {
  let dt = time.delta_secs();

  for (entity, mut transform, mut line, mut sprite) in &mut query {
    line.lifetime -= dt;
    if line.lifetime <= 0.0 {
      commands.entity(entity).despawn();
      continue;
    }

    // Move
    transform.translation.x += line.velocity.x * dt;
    transform.translation.y += line.velocity.y * dt;

    // Fade out
    let alpha = (line.lifetime / line.max_lifetime) * 0.3;
    sprite.color = sprite.color.with_alpha(alpha);
  }
}
