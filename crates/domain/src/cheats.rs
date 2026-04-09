use crate::CheatDefaults;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheatEffect {
  Invincibility(f32),
  SpeedMultiplier(f32, f32), // (multiplier, duration_secs)
  ScoreMultiplier(f32, f32),
  GrowInstant(usize),
  Noclip(f32),
}

#[derive(Debug, Clone)]
pub struct CheatCode {
  name: String,
  effect: CheatEffect,
}

impl CheatCode {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn effect(&self) -> &CheatEffect {
    &self.effect
  }
}

/// Built-in cheat registry. Pure Rust, no external deps.
pub struct CheatRegistry {
  codes: HashMap<String, CheatCode>,
}

impl CheatRegistry {
  pub fn with_defaults(cfg: &CheatDefaults) -> Self {
    let mut codes = HashMap::new();

    let defaults = [
      ("IDDQD", CheatEffect::Invincibility(cfg.invincibility_duration)),
      ("IDKFA", CheatEffect::GrowInstant(cfg.grow_amount)),
      (
        "IMPULSE9",
        CheatEffect::SpeedMultiplier(cfg.speed_multiplier, cfg.speed_duration),
      ),
      ("NOCLIP", CheatEffect::Noclip(cfg.noclip_duration)),
      (
        "BIGHEAD",
        CheatEffect::ScoreMultiplier(cfg.score_multiplier, cfg.score_duration),
      ),
    ];

    for (code, effect) in defaults {
      let name = code.to_string();
      codes.insert(name.clone(), CheatCode { name, effect });
    }

    Self { codes }
  }

  pub fn try_activate(&self, input: &str) -> Option<&CheatEffect> {
    self.codes.get(&input.to_uppercase()).map(|c| &c.effect)
  }

  pub fn list(&self) -> impl Iterator<Item = &CheatCode> {
    self.codes.values()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::CheatDefaults;

  fn registry() -> CheatRegistry {
    CheatRegistry::with_defaults(&CheatDefaults::default())
  }

  #[test]
  fn iddqd_activates_invincibility() {
    let reg = registry();
    let effect = reg.try_activate("IDDQD");
    assert!(effect.is_some());
    assert!(matches!(effect.unwrap(), CheatEffect::Invincibility(_)));
  }

  #[test]
  fn case_insensitive_activation() {
    let reg = registry();
    assert!(reg.try_activate("iddqd").is_some());
    assert!(reg.try_activate("Iddqd").is_some());
  }

  #[test]
  fn unknown_code_returns_none() {
    let reg = registry();
    assert!(reg.try_activate("NOTACHEAT").is_none());
  }

  #[test]
  fn idkfa_activates_grow() {
    let reg = registry();
    let effect = reg.try_activate("IDKFA").unwrap();
    assert!(matches!(effect, CheatEffect::GrowInstant(_)));
  }

  #[test]
  fn impulse9_activates_speed() {
    let reg = registry();
    let effect = reg.try_activate("IMPULSE9").unwrap();
    assert!(matches!(effect, CheatEffect::SpeedMultiplier(_, _)));
  }

  #[test]
  fn noclip_activates_noclip() {
    let reg = registry();
    let effect = reg.try_activate("NOCLIP").unwrap();
    assert!(matches!(effect, CheatEffect::Noclip(_)));
  }

  #[test]
  fn list_returns_all_cheats() {
    let reg = registry();
    let count = reg.list().count();
    assert_eq!(count, 5);
  }
}
