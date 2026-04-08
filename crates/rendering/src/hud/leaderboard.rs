use app::GameWorld;
use bevy::prelude::*;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

const MAX_ENTRIES: usize = 10;

#[derive(Component)]
struct LeaderboardRoot;

#[derive(Component)]
struct LeaderboardEntry {
  rank: usize,
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
        row_gap: Val::Px(2.0),
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

      // 10 entry slots
      for i in 0..MAX_ENTRIES {
        parent.spawn((
          Text::new(""),
          TextFont {
            font: fonts.regular.clone(),
            font_size: 11.0,
            ..default()
          },
          TextColor(theme.fg),
          Visibility::Hidden,
          LeaderboardEntry { rank: i },
        ));
      }
    });
}

fn update_leaderboard(
  world: Option<Res<GameWorld>>,
  theme: Res<GruvboxTheme>,
  mut entries: Query<(&LeaderboardEntry, &mut Text, &mut TextColor, &mut Visibility)>,
) {
  let Some(world) = world else {
    for (_, _, _, mut vis) in &mut entries {
      *vis = Visibility::Hidden;
    }
    return;
  };

  // Collect all alive worms with (name, length, is_player)
  let mut worms: Vec<(&str, usize, bool)> = Vec::new();

  let player = world.player();
  if player.is_alive() {
    worms.push((player.name(), player.length(), true));
  }

  for (worm, _) in world.ai_worms() {
    if worm.is_alive() {
      worms.push((worm.name(), worm.length(), false));
    }
  }

  // Sort by length descending
  worms.sort_by(|a, b| b.1.cmp(&a.1));

  for (entry, mut text, mut color, mut vis) in &mut entries {
    if entry.rank < worms.len() {
      let (name, length, is_player) = worms[entry.rank];
      let rank = entry.rank + 1;
      let marker = if is_player {
        ">"
      } else if rank == 1 {
        "*"
      } else {
        " "
      };
      **text = format!("{}{:>2}. {:<10} {}", marker, rank, name, length);
      color.0 = if is_player {
        theme.green
      } else if rank == 1 {
        theme.yellow
      } else {
        theme.fg.with_alpha(0.7)
      };
      *vis = Visibility::Inherited;
    } else {
      *vis = Visibility::Hidden;
    }
  }
}
