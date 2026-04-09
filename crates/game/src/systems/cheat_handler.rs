use app::GameWorld;
use bevy::prelude::*;
use domain::{CheatDefaults, CheatRegistry};
use rendering::hud::CheatConsole;

use crate::state::AppState;

pub(crate) struct CheatHandlerPlugin;

impl Plugin for CheatHandlerPlugin {
  fn build(&self, app: &mut App) {
    let defaults = CheatDefaults::default();
    app
      .insert_resource(CheatRegistryRes(CheatRegistry::with_defaults(&defaults)))
      .add_systems(Update, process_cheat_input.run_if(in_state(AppState::Playing)));
  }
}

#[derive(Resource)]
struct CheatRegistryRes(CheatRegistry);

fn process_cheat_input(
  mut console: ResMut<CheatConsole>,
  registry: Res<CheatRegistryRes>,
  mut world: Option<ResMut<GameWorld>>,
) {
  let Some(code) = console.take_submitted() else {
    return;
  };

  let Some(ref mut world) = world else { return };

  if let Some(effect) = registry.0.try_activate(&code) {
    world.activate_cheat(effect.clone());
    eprintln!("[cheat] activated: {code}");
  } else {
    eprintln!("[cheat] unknown code: {code}");
  }
}
