use app::GameWorld;
use bevy::prelude::*;
use domain::{ScoreHistory, ScorePersistence, ScoreRecord};
use infra::{data_dir, JsonScoreStore};

use crate::state::AppState;

#[derive(Resource)]
pub(crate) struct ScoreHistoryRes(pub ScoreHistory);

#[derive(Resource)]
struct ScoreStoreRes(JsonScoreStore);

pub(crate) struct PersistencePlugin;

impl Plugin for PersistencePlugin {
  fn build(&self, app: &mut App) {
    let store = JsonScoreStore::new(data_dir());
    let history = store.load().unwrap_or_default();

    app
      .insert_resource(ScoreHistoryRes(history))
      .insert_resource(ScoreStoreRes(store))
      .add_systems(OnEnter(AppState::GameOver), save_score);
  }
}

fn save_score(world: Option<Res<GameWorld>>, mut history: ResMut<ScoreHistoryRes>, store: Res<ScoreStoreRes>) {
  let Some(world) = world else { return };

  let player = world.player();
  let record = ScoreRecord::new(player.score(), player.length(), current_timestamp());

  history.0.add(record);

  if let Err(e) = store.0.save(&history.0) {
    eprintln!("[persistence] save failed: {e}");
  }
}

fn current_timestamp() -> i64 {
  #[cfg(target_arch = "wasm32")]
  {
    0 // simplified for WASM
  }
  #[cfg(not(target_arch = "wasm32"))]
  {
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map(|d| d.as_secs() as i64)
      .unwrap_or(0)
  }
}
