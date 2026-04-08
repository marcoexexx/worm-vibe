use bevy::prelude::*;
use domain::DifficultyPreset;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub enum MenuButton {
  StartEasy,
  StartNormal,
  StartHard,
  Quit,
}

/// Resource: which difficulty the player picked.
#[derive(Resource, Default)]
pub struct SelectedDifficulty(pub Option<DifficultyPreset>);

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<SelectedDifficulty>();
  }
}

/// Spawn the main menu UI. Called by game crate on state enter.
pub fn spawn_main_menu(commands: &mut Commands, theme: &GruvboxTheme, fonts: &GameFonts) {
  let font = fonts.bold.clone();
  let font_regular = fonts.regular.clone();

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
      MainMenuRoot,
    ))
    .with_children(|parent| {
      // Title
      parent.spawn((
        Text::new("WORMZONE"),
        TextFont {
          font: font.clone(),
          font_size: 48.0,
          ..default()
        },
        TextColor(theme.green),
      ));

      // Subtitle
      parent.spawn((
        Text::new("// a developer's snake game"),
        TextFont {
          font: font_regular.clone(),
          font_size: 16.0,
          ..default()
        },
        TextColor(theme.gray),
      ));

      // Spacer
      parent.spawn(Node {
        height: Val::Px(16.0),
        ..default()
      });

      // Difficulty label
      parent.spawn((
        Text::new("SELECT DIFFICULTY"),
        TextFont {
          font: font_regular.clone(),
          font_size: 14.0,
          ..default()
        },
        TextColor(theme.fg),
      ));

      // Difficulty row
      parent
        .spawn(Node {
          flex_direction: FlexDirection::Row,
          column_gap: Val::Px(12.0),
          ..default()
        })
        .with_children(|row| {
          spawn_menu_button(
            row,
            theme,
            &font_regular,
            "[ EASY ]",
            MenuButton::StartEasy,
            theme.green,
          );
          spawn_menu_button(
            row,
            theme,
            &font_regular,
            "[ NORMAL ]",
            MenuButton::StartNormal,
            theme.yellow,
          );
          spawn_menu_button(row, theme, &font_regular, "[ HARD ]", MenuButton::StartHard, theme.red);
        });

      // Spacer
      parent.spawn(Node {
        height: Val::Px(8.0),
        ..default()
      });

      // Quit button
      spawn_menu_button(parent, theme, &font_regular, "[ QUIT ]", MenuButton::Quit, theme.gray);
    });
}

fn spawn_menu_button(
  parent: &mut ChildBuilder,
  theme: &GruvboxTheme,
  font: &Handle<Font>,
  label: &str,
  button: MenuButton,
  text_color: Color,
) {
  parent
    .spawn((
      Button,
      Node {
        padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
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
          font_size: 20.0,
          ..default()
        },
        TextColor(text_color),
      ));
    });
}

/// Despawn main menu entities.
pub fn despawn_main_menu(commands: &mut Commands, query: &Query<Entity, With<MainMenuRoot>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
