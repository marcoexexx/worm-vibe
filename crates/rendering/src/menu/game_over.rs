use bevy::prelude::*;
use domain::ScoreHistory;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

#[derive(Component)]
pub struct GameOverRoot;

#[derive(Component)]
pub enum GameOverButton {
  Retry,
  MainMenu,
}

pub(crate) struct GameOverPlugin;

impl Plugin for GameOverPlugin {
  fn build(&self, _app: &mut App) {}
}

pub fn spawn_game_over(
  commands: &mut Commands,
  theme: &GruvboxTheme,
  fonts: &GameFonts,
  final_score: u64,
  final_length: usize,
  history: &ScoreHistory,
) {
  let font = fonts.regular.clone();
  let font_bold = fonts.bold.clone();

  commands
    .spawn((
      Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(16.0),
        ..default()
      },
      BackgroundColor(theme.bg_hard),
      GameOverRoot,
    ))
    .with_children(|parent| {
      parent.spawn((
        Text::new("GAME OVER"),
        TextFont {
          font: font_bold,
          font_size: 40.0,
          ..default()
        },
        TextColor(theme.red),
      ));

      parent.spawn((
        Text::new(format!("SCORE: {:08}  |  LENGTH: {:03}", final_score, final_length)),
        TextFont {
          font: font.clone(),
          font_size: 20.0,
          ..default()
        },
        TextColor(theme.fg),
      ));

      // Score history table
      let mut table = String::from("--- HIGH SCORES ---\n");
      for (rank, record) in history.top_n(5).iter().enumerate() {
        table.push_str(&format!(
          " {:02}. {:08}  len:{:03}\n",
          rank + 1,
          record.score(),
          record.length(),
        ));
      }

      parent.spawn((
        Text::new(table),
        TextFont {
          font: font.clone(),
          font_size: 14.0,
          ..default()
        },
        TextColor(theme.aqua),
      ));

      spawn_button(parent, theme, &font, "[ RETRY ]", GameOverButton::Retry);
      spawn_button(parent, theme, &font, "[ MAIN_MENU ]", GameOverButton::MainMenu);
    });
}

fn spawn_button(
  parent: &mut ChildBuilder,
  theme: &GruvboxTheme,
  font: &Handle<Font>,
  label: &str,
  button: GameOverButton,
) {
  parent
    .spawn((
      Button,
      Node {
        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
        ..default()
      },
      BackgroundColor(theme.bg_soft),
      button,
    ))
    .with_children(|btn| {
      btn.spawn((
        Text::new(label.to_string()),
        TextFont {
          font: font.clone(),
          font_size: 18.0,
          ..default()
        },
        TextColor(theme.fg),
      ));
    });
}

pub fn despawn_game_over(commands: &mut Commands, query: &Query<Entity, With<GameOverRoot>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
