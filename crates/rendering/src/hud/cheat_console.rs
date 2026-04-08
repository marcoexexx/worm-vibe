use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::fonts::GameFonts;
use crate::theme::GruvboxTheme;

/// Cheat console state — toggled with backtick.
#[derive(Resource, Default)]
pub struct CheatConsole {
  visible: bool,
  buffer: String,
  submitted: Option<String>,
}

impl CheatConsole {
  pub fn is_visible(&self) -> bool {
    self.visible
  }

  pub fn toggle(&mut self) {
    self.visible = !self.visible;
    if !self.visible {
      self.buffer.clear();
    }
  }

  pub fn buffer(&self) -> &str {
    &self.buffer
  }

  pub fn push_char(&mut self, c: char) {
    if c.is_ascii_alphanumeric() || c == '_' {
      self.buffer.push(c);
    }
  }

  pub fn backspace(&mut self) {
    self.buffer.pop();
  }

  pub fn submit(&mut self) {
    if !self.buffer.is_empty() {
      self.submitted = Some(self.buffer.clone());
      self.buffer.clear();
    }
  }

  /// Take the submitted code (consumed on read).
  pub fn take_submitted(&mut self) -> Option<String> {
    self.submitted.take()
  }
}

#[derive(Component)]
struct ConsoleBackground;

#[derive(Component)]
struct ConsoleInputText;

pub struct CheatConsolePlugin;

impl Plugin for CheatConsolePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<CheatConsole>()
      .add_systems(Startup, spawn_console_ui)
      .add_systems(Update, (handle_console_toggle, update_console_display));
  }
}

fn spawn_console_ui(mut commands: Commands, theme: Res<GruvboxTheme>, fonts: Res<GameFonts>) {
  // Console background
  commands
    .spawn((
      Node {
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        width: Val::Percent(100.0),
        height: Val::Px(40.0),
        padding: UiRect::all(Val::Px(8.0)),
        ..default()
      },
      BackgroundColor(theme.bg_hard.with_alpha(0.9)),
      Visibility::Hidden,
      ConsoleBackground,
    ))
    .with_children(|parent| {
      parent.spawn((
        Text::new("cheat> _"),
        TextFont {
          font: fonts.regular.clone(),
          font_size: 16.0,
          ..default()
        },
        TextColor(theme.aqua),
        ConsoleInputText,
      ));
    });
}

fn handle_console_toggle(mut key_events: EventReader<KeyboardInput>, mut console: ResMut<CheatConsole>) {
  for event in key_events.read() {
    if !event.state.is_pressed() {
      continue;
    }
    // Toggle on physical backtick key
    if event.key_code == KeyCode::Backquote {
      console.toggle();
      continue;
    }
    if !console.is_visible() {
      continue;
    }
    match &event.logical_key {
      Key::Enter => {
        console.submit();
      }
      Key::Backspace => {
        console.backspace();
      }
      Key::Character(s) => {
        for c in s.chars() {
          if c != '`' {
            console.push_char(c);
          }
        }
      }
      _ => {}
    }
  }
}

fn update_console_display(
  console: Res<CheatConsole>,
  mut bg_query: Query<&mut Visibility, With<ConsoleBackground>>,
  mut text_query: Query<&mut Text, With<ConsoleInputText>>,
) {
  let Ok(mut vis) = bg_query.get_single_mut() else {
    return;
  };
  *vis = if console.is_visible() {
    Visibility::Visible
  } else {
    Visibility::Hidden
  };

  if let Ok(mut text) = text_query.get_single_mut() {
    **text = format!("cheat> {}_", console.buffer());
  }
}
