use bevy::prelude::*;
use rendering::audio::play_sfx;
use rendering::menu::game_over::{self, GameOverButton, GameOverRoot};
use rendering::menu::main_menu::{self, MainMenuRoot};
use rendering::menu::pause_menu::{self, PauseButton, PauseMenuRoot};
use rendering::theme::GruvboxTheme;
use rendering::{GameFonts, GameSounds};

use crate::state::AppState;
use crate::systems::persistence::ScoreHistoryRes;

pub(crate) struct MenuHandlerPlugin;

impl Plugin for MenuHandlerPlugin {
  fn build(&self, app: &mut App) {
    app
      // Main menu
      .add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
      .add_systems(OnExit(AppState::MainMenu), exit_main_menu)
      .add_systems(Update, handle_main_menu_buttons.run_if(in_state(AppState::MainMenu)))
      // Pause
      .add_systems(OnEnter(AppState::Paused), enter_pause)
      .add_systems(OnExit(AppState::Paused), exit_pause)
      .add_systems(Update, handle_pause_buttons.run_if(in_state(AppState::Paused)))
      // Game over
      .add_systems(OnEnter(AppState::GameOver), enter_game_over)
      .add_systems(OnExit(AppState::GameOver), exit_game_over)
      .add_systems(Update, handle_game_over_buttons.run_if(in_state(AppState::GameOver)));
  }
}

// --- Main Menu ---

fn enter_main_menu(mut commands: Commands, theme: Res<GruvboxTheme>, fonts: Res<GameFonts>) {
  main_menu::spawn_main_menu(&mut commands, &theme, &fonts);
}

fn exit_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
  main_menu::despawn_main_menu(&mut commands, &query);
}

fn handle_main_menu_buttons(
  mut commands: Commands,
  interaction: Query<(&Interaction, &main_menu::MenuButton), Changed<Interaction>>,
  mut next_state: ResMut<NextState<AppState>>,
  mut selected: ResMut<main_menu::SelectedDifficulty>,
  sounds: Option<Res<GameSounds>>,
  #[cfg(not(target_arch = "wasm32"))] mut exit_events: EventWriter<AppExit>,
) {
  for (interaction, button) in &interaction {
    if *interaction != Interaction::Pressed {
      continue;
    }
    if let Some(ref sounds) = sounds {
      play_sfx(&mut commands, &sounds.menu_click);
    }
    match button {
      main_menu::MenuButton::StartEasy => {
        selected.0 = Some(domain::DifficultyPreset::Easy);
        next_state.set(AppState::Playing);
      }
      main_menu::MenuButton::StartNormal => {
        selected.0 = Some(domain::DifficultyPreset::Normal);
        next_state.set(AppState::Playing);
      }
      main_menu::MenuButton::StartHard => {
        selected.0 = Some(domain::DifficultyPreset::Hard);
        next_state.set(AppState::Playing);
      }
      main_menu::MenuButton::Quit => {
        #[cfg(not(target_arch = "wasm32"))]
        exit_events.send(AppExit::Success);
      }
    }
  }
}

// --- Pause ---

fn enter_pause(mut commands: Commands, theme: Res<GruvboxTheme>, fonts: Res<GameFonts>) {
  pause_menu::spawn_pause_menu(&mut commands, &theme, &fonts);
}

fn exit_pause(mut commands: Commands, query: Query<Entity, With<PauseMenuRoot>>) {
  pause_menu::despawn_pause_menu(&mut commands, &query);
}

fn handle_pause_buttons(
  mut commands: Commands,
  interaction: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
  mut next_state: ResMut<NextState<AppState>>,
  sounds: Option<Res<GameSounds>>,
) {
  for (interaction, button) in &interaction {
    if *interaction != Interaction::Pressed {
      continue;
    }
    if let Some(ref sounds) = sounds {
      play_sfx(&mut commands, &sounds.menu_click);
    }
    match button {
      PauseButton::Resume => next_state.set(AppState::Playing),
      PauseButton::MainMenu => next_state.set(AppState::MainMenu),
    }
  }
}

// --- Game Over ---

fn enter_game_over(
  mut commands: Commands,
  theme: Res<GruvboxTheme>,
  fonts: Res<GameFonts>,
  world: Option<Res<app::GameWorld>>,
  history: Option<Res<ScoreHistoryRes>>,
) {
  let (score, length) = world
    .map(|w| (w.player().score(), w.player().length()))
    .unwrap_or((0, 0));

  let empty = domain::ScoreHistory::new();
  let hist = history.as_ref().map(|h| &h.0).unwrap_or(&empty);

  game_over::spawn_game_over(&mut commands, &theme, &fonts, score, length, hist);
}

fn exit_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
  game_over::despawn_game_over(&mut commands, &query);
}

fn handle_game_over_buttons(
  mut commands: Commands,
  interaction: Query<(&Interaction, &GameOverButton), Changed<Interaction>>,
  mut next_state: ResMut<NextState<AppState>>,
  sounds: Option<Res<GameSounds>>,
) {
  for (interaction, button) in &interaction {
    if *interaction != Interaction::Pressed {
      continue;
    }
    if let Some(ref sounds) = sounds {
      play_sfx(&mut commands, &sounds.menu_click);
    }
    match button {
      GameOverButton::Retry => next_state.set(AppState::Playing),
      GameOverButton::MainMenu => next_state.set(AppState::MainMenu),
    }
  }
}
