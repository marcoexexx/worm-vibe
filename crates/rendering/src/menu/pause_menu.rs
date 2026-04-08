use bevy::prelude::*;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component)]
pub enum PauseButton {
  Resume,
  MainMenu,
}

pub(crate) struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
  fn build(&self, _app: &mut App) {}
}

pub fn spawn_pause_menu(commands: &mut Commands, theme: &GruvboxTheme, fonts: &GameFonts) {
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
        row_gap: Val::Px(20.0),
        ..default()
      },
      BackgroundColor(theme.bg_hard.with_alpha(0.85)),
      PauseMenuRoot,
    ))
    .with_children(|parent| {
      parent.spawn((
        Text::new("|| PAUSED"),
        TextFont {
          font: font_bold,
          font_size: 36.0,
          ..default()
        },
        TextColor(theme.yellow),
      ));

      spawn_button(parent, theme, &font, "[ RESUME ]", PauseButton::Resume);
      spawn_button(parent, theme, &font, "[ MAIN_MENU ]", PauseButton::MainMenu);
    });
}

fn spawn_button(
  parent: &mut ChildBuilder,
  theme: &GruvboxTheme,
  font: &Handle<Font>,
  label: &str,
  button: PauseButton,
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

pub fn despawn_pause_menu(commands: &mut Commands, query: &Query<Entity, With<PauseMenuRoot>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
