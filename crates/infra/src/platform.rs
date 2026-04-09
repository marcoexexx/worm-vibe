use std::path::PathBuf;

/// Platform-appropriate data directory for persistent storage.
pub fn data_dir() -> PathBuf {
  #[cfg(target_os = "android")]
  {
    // Android internal storage — set by the activity
    PathBuf::from("/data/data/com.wormzone.game/files")
  }

  #[cfg(target_arch = "wasm32")]
  {
    // WASM uses localStorage via serde, this path is a key prefix
    PathBuf::from("wormzone")
  }

  #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
  {
    dirs::data_local_dir()
      .unwrap_or_else(|| PathBuf::from("."))
      .join("wormzone")
  }
}
