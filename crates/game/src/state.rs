use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
  #[default]
  MainMenu,
  Playing,
  Paused,
  GameOver,
}
