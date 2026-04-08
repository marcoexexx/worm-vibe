use bevy::prelude::*;
use domain::FoodKind;
use rendering::audio::play_sfx;
use rendering::effects::death::{spawn_death_burst, spawn_death_flash};
use rendering::effects::eat::spawn_eat_pop;
use rendering::theme::GruvboxTheme;
use rendering::{GameSounds, ScreenShake};

use crate::state::AppState;
use crate::systems::game_tick::{BoostEvent, FoodEatenEvent, WormDiedEvent};

pub(crate) struct EffectsHandlerPlugin;

impl Plugin for EffectsHandlerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (handle_food_eaten, handle_worm_died, handle_boost_sound).run_if(in_state(AppState::Playing)),
    );
  }
}

fn handle_food_eaten(
  mut commands: Commands,
  mut events: EventReader<FoodEatenEvent>,
  theme: Res<GruvboxTheme>,
  sounds: Option<Res<GameSounds>>,
  world: Option<Res<app::GameWorld>>,
) {
  let player_id = world.as_ref().map(|w| w.player().id());

  for event in events.read() {
    let color = match event.kind {
      FoodKind::Donut => theme.purple,
      FoodKind::Cookie => theme.yellow,
      FoodKind::Cherry => theme.red,
      FoodKind::Banana => theme.yellow,
      FoodKind::Apple => theme.red,
      FoodKind::Grape => theme.purple,
      FoodKind::Watermelon => theme.green,
      FoodKind::Strawberry => theme.red,
    };
    spawn_eat_pop(&mut commands, event.position, color);

    // Only play sound for the player, not NPCs
    if player_id == Some(event.worm_id) {
      if let Some(ref sounds) = sounds {
        play_sfx(&mut commands, &sounds.eat);
      }
    }
  }
}

fn handle_worm_died(
  mut commands: Commands,
  mut events: EventReader<WormDiedEvent>,
  theme: Res<GruvboxTheme>,
  world: Option<Res<app::GameWorld>>,
  mut shake: ResMut<ScreenShake>,
  sounds: Option<Res<GameSounds>>,
) {
  for event in events.read() {
    // Particle burst
    spawn_death_burst(&mut commands, event.position, 20, theme.orange);

    // Only flash + shake + sound if the player died
    let is_player = world
      .as_ref()
      .map(|w| w.player().id() == event.worm_id)
      .unwrap_or(false);

    if is_player {
      spawn_death_flash(&mut commands, &theme);
      shake.trigger(8.0, 0.3);
      if let Some(ref sounds) = sounds {
        play_sfx(&mut commands, &sounds.die);
      }
    }
  }
}

fn handle_boost_sound(
  mut commands: Commands,
  mut events: EventReader<BoostEvent>,
  world: Option<Res<app::GameWorld>>,
  sounds: Option<Res<GameSounds>>,
) {
  let Some(ref sounds) = sounds else { return };
  let Some(world) = world else { return };

  for event in events.read() {
    // Only play sound for player's boost
    if event.started && event.worm_id == world.player().id() {
      play_sfx(&mut commands, &sounds.boost_start);
    }
  }
}
