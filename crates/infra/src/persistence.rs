use domain::{ScoreHistory, ScorePersistence};
use std::path::PathBuf;

pub struct JsonScoreStore {
  path: PathBuf,
}

impl JsonScoreStore {
  pub fn new(dir: PathBuf) -> Self {
    Self {
      path: dir.join("scores.json"),
    }
  }
}

impl ScorePersistence for JsonScoreStore {
  fn load(&self) -> Result<ScoreHistory, String> {
    #[cfg(target_arch = "wasm32")]
    {
      // WASM: use localStorage
      Ok(ScoreHistory::new())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
      if !self.path.exists() {
        return Ok(ScoreHistory::new());
      }
      let data = std::fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
      serde_json::from_str(&data).map_err(|e| e.to_string())
    }
  }

  fn save(&self, history: &ScoreHistory) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
      let _ = history;
      // WASM: would use localStorage
      Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
      if let Some(parent) = self.path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
      }
      let data = serde_json::to_string_pretty(history).map_err(|e| e.to_string())?;
      std::fs::write(&self.path, data).map_err(|e| e.to_string())
    }
  }
}
