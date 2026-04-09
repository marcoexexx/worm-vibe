//! Settings menu UI — presentation only.
//!
//! All settings data lives in `domain::GameSettings`.
//! This module owns the Bevy `Resource` wrapper and the spawn/despawn UI.

use bevy::prelude::*;
use domain::{BoostSide, ControlMode, GameSettings};

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

// ============================================================================
// Bevy resource wrapper
// ============================================================================

/// Newtype so `domain::GameSettings` can be a Bevy `Resource`.
#[derive(Resource, Clone, Debug, Default, Deref, DerefMut)]
pub struct SettingsRes(pub GameSettings);

// ============================================================================
// Components
// ============================================================================

#[derive(Component)]
pub struct SettingsMenuRoot;

#[derive(Component)]
pub enum SettingsAction {
  SetControl(ControlMode),
  SetBoost(BoostSide),
  SetSound(bool),
  ZoomIn,
  ZoomOut,
  Back,
}

#[derive(Component)]
struct ZoomLabel;

/// Tracks which option this card represents, for visual refresh.
#[derive(Component)]
struct SelectionCard(SelectionKey);

/// Marker on the primary text child of a card/pill, for color refresh.
#[derive(Component)]
struct CardText;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SelectionKey {
  Control(ControlMode),
  Boost(BoostSide),
  Sound(bool),
}

impl SelectionKey {
  fn is_selected(self, s: &GameSettings) -> bool {
    match self {
      Self::Control(m) => s.control_mode == m,
      Self::Boost(b) => s.boost_side == b,
      Self::Sound(on) => s.sound_on == on,
    }
  }

  /// Pills (boost, sound) use filled-green style; cards use bordered style.
  fn is_pill(self) -> bool {
    matches!(self, Self::Boost(_) | Self::Sound(_))
  }
}

// ============================================================================
// Plugin
// ============================================================================

pub(crate) struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<SettingsRes>()
      .add_systems(Update, refresh_visuals.run_if(resource_changed::<SettingsRes>));
  }
}

// ============================================================================
// Spawn / Despawn
// ============================================================================

pub fn spawn_settings_menu(commands: &mut Commands, theme: &GruvboxTheme, fonts: &GameFonts, settings: &GameSettings) {
  let bold = &fonts.bold;
  let regular = &fonts.regular;

  commands
    .spawn((fullscreen_center(), BackgroundColor(theme.bg_hard), SettingsMenuRoot))
    .with_children(|root| {
      root.spawn(panel(theme)).with_children(|p| {
        // Title
        label(p, "SETTINGS", bold, 32.0, theme.fg);

        // ── CONTROLS ──
        heading(p, bold, "CONTROLS", theme.aqua);
        p.spawn(row(12.0)).with_children(|r| {
          for mode in ControlMode::ALL {
            card(
              r,
              theme,
              regular,
              mode.label(),
              Some(mode.hint()),
              SettingsAction::SetControl(mode),
              SelectionKey::Control(mode),
              settings.control_mode == mode,
            );
          }
        });

        // ── SPEED BOOST ──
        heading(p, bold, "SPEED BOOST", theme.aqua);
        p.spawn(row(12.0)).with_children(|r| {
          for side in BoostSide::ALL {
            pill(
              r,
              theme,
              bold,
              side.label(),
              SettingsAction::SetBoost(side),
              SelectionKey::Boost(side),
              settings.boost_side == side,
            );
          }
        });

        // ── SOUND ──
        heading(p, bold, "SOUND", theme.aqua);
        p.spawn(row(12.0)).with_children(|r| {
          for &(text, value) in &[("ON", true), ("OFF", false)] {
            pill(
              r,
              theme,
              bold,
              text,
              SettingsAction::SetSound(value),
              SelectionKey::Sound(value),
              settings.sound_on == value,
            );
          }
        });

        // ── ZOOM ──
        heading(p, bold, "ZOOM", theme.aqua);
        p.spawn(row_centered(12.0)).with_children(|r| {
          icon_btn(r, theme, bold, "-", SettingsAction::ZoomOut);
          r.spawn((
            Text::new(fmt_zoom(settings.zoom_level)),
            TextFont {
              font: bold.clone(),
              font_size: 20.0,
              ..default()
            },
            TextColor(theme.fg),
            Node {
              width: Val::Px(60.0),
              ..default()
            },
            ZoomLabel,
          ));
          icon_btn(r, theme, bold, "+", SettingsAction::ZoomIn);
        });

        // Spacer + Back
        p.spawn(Node {
          height: Val::Px(8.0),
          ..default()
        });
        p.spawn((
          Button,
          Node {
            padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
            ..default()
          },
          BackgroundColor(theme.bg_soft),
          SettingsAction::Back,
        ))
        .with_children(|b| label(b, "[ BACK ]", regular, 20.0, theme.fg));
      });
    });
}

pub fn despawn_settings_menu(commands: &mut Commands, query: &Query<Entity, With<SettingsMenuRoot>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}

// ============================================================================
// Layout primitives
// ============================================================================

fn fullscreen_center() -> Node {
  Node {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
  }
}

fn panel(theme: &GruvboxTheme) -> (Node, BackgroundColor) {
  (
    Node {
      width: Val::Px(420.0),
      flex_direction: FlexDirection::Column,
      align_items: AlignItems::Center,
      padding: UiRect::all(Val::Px(24.0)),
      row_gap: Val::Px(20.0),
      ..default()
    },
    BackgroundColor(theme.bg.with_alpha(0.95)),
  )
}

fn row(gap: f32) -> Node {
  Node {
    flex_direction: FlexDirection::Row,
    column_gap: Val::Px(gap),
    ..default()
  }
}

fn row_centered(gap: f32) -> Node {
  Node {
    flex_direction: FlexDirection::Row,
    column_gap: Val::Px(gap),
    align_items: AlignItems::Center,
    ..default()
  }
}

// ============================================================================
// Widget builders
// ============================================================================

fn label(parent: &mut ChildBuilder, s: &str, font: &Handle<Font>, size: f32, color: Color) {
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

fn heading(parent: &mut ChildBuilder, font: &Handle<Font>, text: &str, color: Color) {
  label(parent, text, font, 14.0, color);
}

/// Bordered option card (ARROW / JOYSTICK / MOUSE style).
#[allow(clippy::too_many_arguments)]
fn card(
  parent: &mut ChildBuilder,
  theme: &GruvboxTheme,
  font: &Handle<Font>,
  title: &str,
  subtitle: Option<&str>,
  action: SettingsAction,
  key: SelectionKey,
  selected: bool,
) {
  let (bg, border, fg) = card_colors(theme, selected);
  parent
    .spawn((
      Button,
      Node {
        width: Val::Px(115.0),
        height: Val::Px(80.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        row_gap: Val::Px(4.0),
        ..default()
      },
      BackgroundColor(bg),
      BorderColor(border),
      action,
      SelectionCard(key),
    ))
    .with_children(|c| {
      c.spawn((
        Text::new(title),
        TextFont {
          font: font.clone(),
          font_size: 14.0,
          ..default()
        },
        TextColor(fg),
        CardText,
      ));
      if let Some(sub) = subtitle {
        label(c, sub, font, 9.0, theme.gray);
      }
    });
}

/// Toggle pill (LEFT/RIGHT, ON/OFF style).
fn pill(
  parent: &mut ChildBuilder,
  theme: &GruvboxTheme,
  font: &Handle<Font>,
  text: &str,
  action: SettingsAction,
  key: SelectionKey,
  selected: bool,
) {
  let (bg, border, fg) = pill_colors(theme, selected);
  parent
    .spawn((
      Button,
      Node {
        width: Val::Px(100.0),
        height: Val::Px(36.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        ..default()
      },
      BackgroundColor(bg),
      BorderColor(border),
      action,
      SelectionCard(key),
    ))
    .with_children(|p| {
      p.spawn((
        Text::new(text),
        TextFont {
          font: font.clone(),
          font_size: 16.0,
          ..default()
        },
        TextColor(fg),
        CardText,
      ));
    });
}

/// Small square button (+/-).
fn icon_btn(parent: &mut ChildBuilder, theme: &GruvboxTheme, font: &Handle<Font>, text: &str, action: SettingsAction) {
  parent
    .spawn((
      Button,
      Node {
        width: Val::Px(36.0),
        height: Val::Px(36.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
      },
      BackgroundColor(theme.bg_soft),
      BorderColor(theme.gray.with_alpha(0.4)),
      action,
    ))
    .with_children(|b| label(b, text, font, 22.0, theme.fg));
}

// ============================================================================
// Color helpers
// ============================================================================

/// Returns (bg, border, text_color) for a bordered card.
fn card_colors(theme: &GruvboxTheme, selected: bool) -> (Color, Color, Color) {
  if selected {
    (theme.bg_soft, theme.green, theme.green)
  } else {
    (theme.bg_hard, theme.gray.with_alpha(0.3), theme.fg)
  }
}

/// Returns (bg, border, text_color) for a filled pill.
fn pill_colors(theme: &GruvboxTheme, selected: bool) -> (Color, Color, Color) {
  if selected {
    (theme.green, theme.green, theme.bg_hard)
  } else {
    (theme.bg_soft, theme.gray.with_alpha(0.3), theme.fg)
  }
}

fn fmt_zoom(level: f32) -> String {
  format!("{:.0}%", level * 100.0)
}

// ============================================================================
// Live visual refresh
// ============================================================================

fn refresh_visuals(
  settings: Res<SettingsRes>,
  theme: Res<GruvboxTheme>,
  mut cards: Query<(&SelectionCard, &mut BackgroundColor, &mut BorderColor, &Children)>,
  mut card_texts: Query<&mut TextColor, With<CardText>>,
  mut zoom: Query<&mut Text, With<ZoomLabel>>,
) {
  for (card, mut bg, mut border, children) in &mut cards {
    let sel = card.0.is_selected(&settings);
    let (new_bg, new_border, new_fg) = if card.0.is_pill() {
      pill_colors(&theme, sel)
    } else {
      card_colors(&theme, sel)
    };
    bg.0 = new_bg;
    border.0 = new_border;

    // Propagate text color to CardText children
    for &child in children.iter() {
      if let Ok(mut tc) = card_texts.get_mut(child) {
        tc.0 = new_fg;
      }
    }
  }

  for mut t in &mut zoom {
    **t = fmt_zoom(settings.zoom_level);
  }
}
