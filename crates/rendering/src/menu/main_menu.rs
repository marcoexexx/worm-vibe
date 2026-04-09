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
  Settings,
  About,
  Quit,
}

#[derive(Resource, Default)]
pub struct SelectedDifficulty(pub Option<DifficultyPreset>);

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<SelectedDifficulty>();
  }
}

pub fn spawn_main_menu(commands: &mut Commands, theme: &GruvboxTheme, fonts: &GameFonts, window: &Window) {
  let bold = fonts.bold.clone();
  let regular = fonts.regular.clone();
  let s = ui_scale(window.width());

  commands
    .spawn((
      Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(12.0 * s),
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
          font: bold.clone(),
          font_size: 42.0 * s,
          ..default()
        },
        TextColor(theme.green),
      ));

      parent.spawn((
        Text::new("// a developer's snake game"),
        TextFont {
          font: regular.clone(),
          font_size: 13.0 * s,
          ..default()
        },
        TextColor(theme.gray),
      ));

      spacer(parent, 12.0 * s);

      parent.spawn((
        Text::new("SELECT DIFFICULTY"),
        TextFont {
          font: regular.clone(),
          font_size: 11.0 * s,
          ..default()
        },
        TextColor(theme.fg),
      ));

      // Difficulty row
      parent
        .spawn(Node {
          flex_direction: FlexDirection::Row,
          column_gap: Val::Px(8.0 * s),
          ..default()
        })
        .with_children(|row| {
          menu_btn(row, theme, &regular, "[ EASY ]", MenuButton::StartEasy, theme.green, s);
          menu_btn(
            row,
            theme,
            &regular,
            "[ NORMAL ]",
            MenuButton::StartNormal,
            theme.yellow,
            s,
          );
          menu_btn(row, theme, &regular, "[ HARD ]", MenuButton::StartHard, theme.red, s);
        });

      spacer(parent, 4.0 * s);

      // Settings / About row
      parent
        .spawn(Node {
          flex_direction: FlexDirection::Row,
          column_gap: Val::Px(8.0 * s),
          ..default()
        })
        .with_children(|row| {
          menu_btn(
            row,
            theme,
            &regular,
            "[ SETTINGS ]",
            MenuButton::Settings,
            theme.aqua,
            s,
          );
          menu_btn(row, theme, &regular, "[ ABOUT ]", MenuButton::About, theme.purple, s);
        });

      if !cfg!(target_arch = "wasm32") {
        menu_btn(parent, theme, &regular, "[ QUIT ]", MenuButton::Quit, theme.gray, s);
      }
    });
}

pub fn despawn_main_menu(commands: &mut Commands, query: &Query<Entity, With<MainMenuRoot>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

/// Scale factor: 1.0 at 1280px, ~0.55 at 360px (mobile)
fn ui_scale(width: f32) -> f32 {
  (width / 1280.0).clamp(0.55, 1.0)
}

fn menu_btn(
  parent: &mut ChildBuilder,
  theme: &GruvboxTheme,
  font: &Handle<Font>,
  label: &str,
  button: MenuButton,
  color: Color,
  scale: f32,
) {
  parent
    .spawn((
      Button,
      Node {
        padding: UiRect::axes(Val::Px(18.0 * scale), Val::Px(8.0 * scale)),
        ..default()
      },
      BackgroundColor(theme.bg_soft),
      button,
    ))
    .with_children(|btn| {
      btn.spawn((
        Text::new(label),
        TextFont {
          font: font.clone(),
          font_size: 16.0 * scale,
          ..default()
        },
        TextColor(color),
      ));
    });
}

fn spacer(parent: &mut ChildBuilder, height: f32) {
  parent.spawn(Node {
    height: Val::Px(height),
    ..default()
  });
}
