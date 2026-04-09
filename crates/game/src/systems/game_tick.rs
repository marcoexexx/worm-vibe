use app::{tick_game_world, GameWorld};
use bevy::prelude::*;
use domain::{DomainEvent, FoodKind, GameConfig, WormId};
use glam::Vec2;
use input::CurrentIntent;
use rendering::menu::main_menu::SelectedDifficulty;

use crate::state::AppState;

// --- Bevy event wrappers for domain events ---

#[derive(Event)]
#[allow(dead_code)]
pub struct FoodEatenEvent {
  pub worm_id: WormId,
  pub position: Vec2,
  pub kind: FoodKind,
  pub new_score: u64,
}

#[derive(Event)]
#[allow(dead_code)]
pub struct WormDiedEvent {
  pub worm_id: WormId,
  pub position: Vec2,
  pub final_score: u64,
}

#[derive(Event)]
#[allow(dead_code)]
pub struct BoostEvent {
  pub worm_id: WormId,
  pub started: bool,
}

pub(crate) struct GameTickPlugin;

impl Plugin for GameTickPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<FoodEatenEvent>()
      .add_event::<WormDiedEvent>()
      .add_event::<BoostEvent>()
      .add_systems(OnEnter(AppState::Playing), setup_game_world)
      .add_systems(
        Update,
        (run_game_tick, check_player_death, handle_pause_input).run_if(in_state(AppState::Playing)),
      )
      .add_systems(Update, handle_unpause_input.run_if(in_state(AppState::Paused)));
  }
}

fn setup_game_world(mut commands: Commands, existing: Option<Res<GameWorld>>, selected: Res<SelectedDifficulty>) {
  if existing.is_some() {
    commands.remove_resource::<GameWorld>();
  }
  let preset = selected.0.unwrap_or(domain::DifficultyPreset::Normal);
  let config = GameConfig::from_preset(preset);
  commands.insert_resource(GameWorld::new(config));
}

fn run_game_tick(
  mut world: ResMut<GameWorld>,
  intent: Res<CurrentIntent>,
  time: Res<Time>,
  mut food_events: EventWriter<FoodEatenEvent>,
  mut death_events: EventWriter<WormDiedEvent>,
  mut boost_events: EventWriter<BoostEvent>,
) {
  let dt = time.delta_secs();
  let events = tick_game_world(&mut world, intent.0, dt);

  // Dispatch domain events to Bevy event bus
  for event in events {
    match event {
      DomainEvent::FoodEaten {
        worm_id,
        position,
        kind,
        new_score,
      } => {
        food_events.send(FoodEatenEvent {
          worm_id,
          position,
          kind,
          new_score,
        });
      }
      DomainEvent::WormDied {
        worm_id,
        position,
        final_score,
        ..
      } => {
        death_events.send(WormDiedEvent {
          worm_id,
          position,
          final_score,
        });
      }
      DomainEvent::BoostStarted { worm_id } => {
        boost_events.send(BoostEvent { worm_id, started: true });
      }
      DomainEvent::BoostEnded { worm_id } => {
        boost_events.send(BoostEvent {
          worm_id,
          started: false,
        });
      }
      _ => {}
    }
  }
}

fn check_player_death(world: Res<GameWorld>, mut next_state: ResMut<NextState<AppState>>) {
  if !world.player().is_alive() {
    next_state.set(AppState::GameOver);
  }
}

fn handle_pause_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
  if keys.just_pressed(KeyCode::Escape) {
    next_state.set(AppState::Paused);
  }
}

fn handle_unpause_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
  if keys.just_pressed(KeyCode::Escape) {
    next_state.set(AppState::Playing);
  }
}
