#![allow(dead_code)]
use domain::{FoodKind, ScoreHistory, ScorePersistence, ScoreRecord};

/// Calculate score gained from eating food.
pub(crate) fn compute_gain(kind: FoodKind) -> u64 {
  kind.score_value()
}

/// Save a completed game to history.
pub(crate) fn record_game(history: &mut ScoreHistory, score: u64, length: usize, persistence: &dyn ScorePersistence) {
  let timestamp = current_timestamp();
  let record = ScoreRecord::new(score, length, timestamp);
  history.add(record);

  if let Err(e) = persistence.save(history) {
    eprintln!("[score] failed to persist: {e}");
  }
}

fn current_timestamp() -> i64 {
  #[cfg(target_arch = "wasm32")]
  {
    (js_sys::Date::now() / 1000.0) as i64
  }
  #[cfg(not(target_arch = "wasm32"))]
  {
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map(|d| d.as_secs() as i64)
      .unwrap_or(0)
  }
}
