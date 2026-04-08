use app::GameWorld;
use bevy::prelude::*;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

/// 10 top entries + 1 separator ("...") + 1 player entry = 12 slots
const MAX_SLOTS: usize = 12;

#[derive(Component)]
struct LeaderboardRoot;

#[derive(Component)]
struct LeaderboardSlot {
  index: usize,
}

pub(crate) struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_leaderboard)
      .add_systems(Update, update_leaderboard);
  }
}

fn spawn_leaderboard(mut commands: Commands, theme: Res<GruvboxTheme>, fonts: Res<GameFonts>) {
  commands
    .spawn((
      Node {
        position_type: PositionType::Absolute,
        top: Val::Px(40.0),
        left: Val::Px(16.0),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(1.0),
        ..default()
      },
      LeaderboardRoot,
    ))
    .with_children(|parent| {
      // Title
      parent.spawn((
        Text::new("TOP 10"),
        TextFont {
          font: fonts.bold.clone(),
          font_size: 13.0,
          ..default()
        },
        TextColor(theme.yellow),
      ));

      // Slots (reusable each frame)
      for i in 0..MAX_SLOTS {
        parent.spawn((
          Text::new(""),
          TextFont {
            font: fonts.regular.clone(),
            font_size: 11.0,
            ..default()
          },
          TextColor(theme.fg),
          Visibility::Hidden,
          LeaderboardSlot { index: i },
        ));
      }
    });
}

/// A single leaderboard display line.
struct DisplayLine {
  text: String,
  color: Color,
}

fn update_leaderboard(
  world: Option<Res<GameWorld>>,
  theme: Res<GruvboxTheme>,
  mut slots: Query<(&LeaderboardSlot, &mut Text, &mut TextColor, &mut Visibility)>,
) {
  let Some(world) = world else {
    for (_, _, _, mut vis) in &mut slots {
      *vis = Visibility::Hidden;
    }
    return;
  };

  // Collect all alive worms: (name, score, is_player)
  let mut worms: Vec<(&str, u64, bool)> = Vec::new();
  let player = world.player();
  if player.is_alive() {
    worms.push((player.name(), player.score(), true));
  }
  for (worm, _) in world.ai_worms() {
    if worm.is_alive() {
      worms.push((worm.name(), worm.score(), false));
    }
  }
  worms.sort_by(|a, b| b.1.cmp(&a.1));

  // Find player's rank (1-based)
  let player_rank = worms.iter().position(|w| w.2).map(|i| i + 1);
  let player_in_top10 = player_rank.map_or(false, |r| r <= 10);

  // Build display lines
  let mut lines: Vec<DisplayLine> = Vec::with_capacity(MAX_SLOTS);

  // Top 10
  let top_count = worms.len().min(10);
  for i in 0..top_count {
    let (name, score, is_player) = worms[i];
    let rank = i + 1;
    let king = if rank == 1 { "K" } else { " " };
    let ptr = if is_player { ">" } else { " " };
    let color = if is_player {
      theme.green
    } else if rank == 1 {
      theme.yellow
    } else {
      theme.fg.with_alpha(0.7)
    };
    lines.push(DisplayLine {
      text: format!("{}{}{:>2} {:<10} {}", ptr, king, rank, name, score),
      color,
    });
  }

  // If player is NOT in top 10, append separator + player line
  if !player_in_top10 {
    if let Some(rank) = player_rank {
      lines.push(DisplayLine {
        text: "  ...".to_string(),
        color: theme.gray,
      });
      let score = player.score();
      lines.push(DisplayLine {
        text: format!("> {:>2} {:<10} {}", rank, player.name(), score),
        color: theme.green,
      });
    }
  }

  // Apply to slots
  for (slot, mut text, mut color, mut vis) in &mut slots {
    if slot.index < lines.len() {
      let line = &lines[slot.index];
      **text = line.text.clone();
      color.0 = line.color;
      *vis = Visibility::Inherited;
    } else {
      *vis = Visibility::Hidden;
    }
  }
}
