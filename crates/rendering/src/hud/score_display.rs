use app::GameWorld;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

#[derive(Component)]
struct ScoreText;

pub struct ScoreDisplayPlugin;

impl Plugin for ScoreDisplayPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(FrameTimeDiagnosticsPlugin)
      .add_systems(Startup, spawn_hud)
      .add_systems(Update, (update_score, scale_hud_to_window));
  }
}

fn spawn_hud(mut commands: Commands, theme: Res<GruvboxTheme>, fonts: Res<GameFonts>) {
  commands.spawn((
    Text::new("> SCORE: 00000000 | LEN: 000 | FPS: --- | 00:00"),
    TextFont {
      font: fonts.regular.clone(),
      font_size: 18.0,
      ..default()
    },
    TextColor(theme.green),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(8.0),
      left: Val::Px(8.0),
      ..default()
    },
    ScoreText,
  ));
}

fn update_score(
  world: Option<Res<GameWorld>>,
  diagnostics: Res<DiagnosticsStore>,
  mut query: Query<&mut Text, With<ScoreText>>,
) {
  let Ok(mut text) = query.get_single_mut() else {
    return;
  };

  let fps = diagnostics
    .get(&FrameTimeDiagnosticsPlugin::FPS)
    .and_then(|d| d.smoothed())
    .unwrap_or(0.0);

  let Some(world) = world else {
    **text = format!("> FPS: {:03}", fps as u32);
    return;
  };

  let score = world.player().score();
  let length = world.player().length();
  let elapsed = world.elapsed();
  let mins = (elapsed / 60.0) as u32;
  let secs = (elapsed % 60.0) as u32;

  **text = format!(
    "> SCORE: {:08} | LEN: {:03} | FPS: {:03} | {:02}:{:02}",
    score, length, fps as u32, mins, secs
  );
}

fn scale_hud_to_window(windows: Query<&Window>, mut query: Query<&mut TextFont, With<ScoreText>>) {
  let Ok(window) = windows.get_single() else {
    return;
  };
  let Ok(mut font) = query.get_single_mut() else {
    return;
  };
  // Scale font: 18px at 1280w, 11px at 400w
  let scale = (window.width() / 1280.0).clamp(0.6, 1.0);
  font.font_size = 18.0 * scale;
}
