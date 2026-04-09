use bevy::prelude::*;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

#[derive(Component)]
pub struct AboutMenuRoot;

#[derive(Component)]
pub struct AboutButton;

pub(crate) struct AboutMenuPlugin;

impl Plugin for AboutMenuPlugin {
  fn build(&self, _app: &mut App) {}
}

pub fn spawn_about_menu(commands: &mut Commands, theme: &GruvboxTheme, fonts: &GameFonts) {
  let bold = &fonts.bold;
  let regular = &fonts.regular;

  commands
    .spawn((
      Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
      },
      BackgroundColor(theme.bg_hard),
      AboutMenuRoot,
    ))
    .with_children(|root| {
      root
        .spawn(Node {
          width: Val::Px(480.0),
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::Center,
          padding: UiRect::all(Val::Px(24.0)),
          row_gap: Val::Px(12.0),
          ..default()
        })
        .with_children(|panel| {
          // Game title
          text(panel, "WORMZONE", bold, 36.0, theme.green);
          text(panel, "v0.1.0", regular, 12.0, theme.gray);

          spacer(panel, 8.0);

          // Description
          section(panel, bold, "ABOUT", theme.aqua);
          text(
            panel,
            "A slither.io-inspired snake game.\nGrow your worm, boost to hunt, become king.",
            regular,
            12.0,
            theme.fg,
          );

          spacer(panel, 4.0);

          // How to play
          section(panel, bold, "HOW TO PLAY", theme.aqua);
          text(
            panel,
            "Eat food to grow longer and gain score.",
            regular,
            11.0,
            theme.fg,
          );
          text(panel, "Make other worms crash into your body.", regular, 11.0, theme.fg);
          text(
            panel,
            "Boost with Space/Shift (drains length).",
            regular,
            11.0,
            theme.fg,
          );
          text(panel, "The longest worm wears the crown.", regular, 11.0, theme.yellow);

          spacer(panel, 4.0);

          // Credits
          section(panel, bold, "CREDITS", theme.aqua);
          text(panel, "Developed by  Aung Koko Lwin", regular, 11.0, theme.fg);
          text(panel, "Engine        Bevy 0.15 (Rust)", regular, 11.0, theme.fg);
          text(panel, "Font          JetBrains Mono", regular, 11.0, theme.fg);
          text(panel, "AI Assist     Claude (Anthropic)", regular, 11.0, theme.fg);

          spacer(panel, 4.0);

          // License
          section(panel, bold, "LICENSE", theme.aqua);
          text(panel, "MIT License (c) 2026 Aung Koko Lwin", regular, 11.0, theme.fg);
          text(panel, "Free to use, modify, and distribute.", regular, 10.0, theme.gray);

          spacer(panel, 4.0);

          // Links
          section(panel, bold, "LINKS", theme.aqua);
          text(panel, "github.com/marcoexexx/worm-vibe", regular, 11.0, theme.blue);

          spacer(panel, 12.0);

          // Back button
          panel
            .spawn((
              Button,
              Node {
                padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                ..default()
              },
              BackgroundColor(theme.bg_soft),
              AboutButton,
            ))
            .with_children(|btn| {
              text(btn, "[ BACK ]", regular, 20.0, theme.fg);
            });
        });
    });
}

pub fn despawn_about_menu(commands: &mut Commands, query: &Query<Entity, With<AboutMenuRoot>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

fn text(parent: &mut ChildBuilder, s: &str, font: &Handle<Font>, size: f32, color: Color) {
  parent.spawn((
    Text::new(s),
    TextFont {
      font: font.clone(),
      font_size: size,
      ..default()
    },
    TextColor(color),
  ));
}

fn section(parent: &mut ChildBuilder, font: &Handle<Font>, label: &str, color: Color) {
  text(parent, label, font, 13.0, color);
}

fn spacer(parent: &mut ChildBuilder, height: f32) {
  parent.spawn(Node {
    height: Val::Px(height),
    ..default()
  });
}
