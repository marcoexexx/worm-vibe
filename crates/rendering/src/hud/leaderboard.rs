use app::GameWorld;
use bevy::prelude::*;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

/// 10 top entries + 1 separator ("...") + 1 player entry = 12 slots
const MAX_SLOTS: usize = 12;
const CROWN_SIZE: f32 = 14.0;

#[derive(Component)]
struct LeaderboardRoot;

#[derive(Component)]
struct LeaderboardSlot {
  index: usize,
}

/// Crown icon within a leaderboard row.
#[derive(Component)]
struct CrownIcon {
  index: usize,
}

/// Text portion of a leaderboard row.
#[derive(Component)]
struct SlotText {
  index: usize,
}

/// Pre-loaded crown texture.
#[derive(Resource)]
struct CrownTexture(Handle<Image>);

pub(crate) struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PreStartup, load_crown)
      .add_systems(Startup, spawn_leaderboard)
      .add_systems(Update, update_leaderboard);
  }
}

fn load_crown(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.insert_resource(CrownTexture(asset_server.load("textures/crown.png")));
}

fn spawn_leaderboard(
  mut commands: Commands,
  theme: Res<GruvboxTheme>,
  fonts: Res<GameFonts>,
  crown_tex: Res<CrownTexture>,
) {
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

      // Each slot is a row: [crown_img] [text]
      for i in 0..MAX_SLOTS {
        parent
          .spawn((
            Node {
              flex_direction: FlexDirection::Row,
              align_items: AlignItems::Center,
              column_gap: Val::Px(2.0),
              height: Val::Px(14.0),
              ..default()
            },
            LeaderboardSlot { index: i },
            Visibility::Hidden,
          ))
          .with_children(|row| {
            // Crown icon (hidden by default)
            row.spawn((
              ImageNode {
                image: crown_tex.0.clone(),
                ..default()
              },
              Node {
                width: Val::Px(CROWN_SIZE),
                height: Val::Px(CROWN_SIZE),
                ..default()
              },
              Visibility::Hidden,
              CrownIcon { index: i },
            ));

            // Text
            row.spawn((
              Text::new(""),
              TextFont {
                font: fonts.regular.clone(),
                font_size: 11.0,
                ..default()
              },
              TextColor(theme.fg),
              SlotText { index: i },
            ));
          });
      }
    });
}

fn update_leaderboard(
  world: Option<Res<GameWorld>>,
  theme: Res<GruvboxTheme>,
  mut slots: Query<(&LeaderboardSlot, &mut Visibility)>,
  mut texts: Query<(&SlotText, &mut Text, &mut TextColor)>,
  mut crowns: Query<(&CrownIcon, &mut Visibility), Without<LeaderboardSlot>>,
) {
  let Some(world) = world else {
    for (_, mut vis) in &mut slots {
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

  let player_rank = worms.iter().position(|w| w.2).map(|i| i + 1);
  let player_in_top10 = player_rank.map_or(false, |r| r <= 10);

  // Build display lines: (text, color, show_crown)
  let mut lines: Vec<(String, Color, bool)> = Vec::with_capacity(MAX_SLOTS);

  let top_count = worms.len().min(10);
  for i in 0..top_count {
    let (name, score, is_player) = worms[i];
    let rank = i + 1;
    let ptr = if is_player { "> " } else { "  " };
    let color = if is_player {
      theme.green
    } else if rank == 1 {
      theme.yellow
    } else {
      theme.fg.with_alpha(0.7)
    };
    lines.push((
      format!("{}{:>2} {:<10} {}", ptr, rank, name, score),
      color,
      rank == 1,
    ));
  }

  if !player_in_top10 {
    if let Some(rank) = player_rank {
      lines.push(("  ...".to_string(), theme.gray, false));
      lines.push((
        format!("> {:>3} {:<10} {}", rank, player.name(), player.score()),
        theme.green,
        false,
      ));
    }
  }

  // Apply to slots
  for (slot, mut vis) in &mut slots {
    *vis = if slot.index < lines.len() {
      Visibility::Inherited
    } else {
      Visibility::Hidden
    };
  }

  for (st, mut text, mut color) in &mut texts {
    if st.index < lines.len() {
      let (ref line, c, _) = lines[st.index];
      **text = line.clone();
      color.0 = c;
    }
  }

  for (ci, mut vis) in &mut crowns {
    *vis = if ci.index < lines.len() && lines[ci.index].2 {
      Visibility::Inherited
    } else {
      Visibility::Hidden
    };
  }
}
