use domain::{CheatEffect, Worm};

#[derive(Debug, Clone)]
pub struct ActiveCheat {
  effect: CheatEffect,
  remaining: Option<f32>,
}

impl ActiveCheat {
  pub fn new(effect: CheatEffect) -> Self {
    let remaining = match &effect {
      CheatEffect::Invincibility(d) => Some(*d),
      CheatEffect::SpeedMultiplier(_, d) => Some(*d),
      CheatEffect::ScoreMultiplier(_, d) => Some(*d),
      CheatEffect::Noclip(d) => Some(*d),
      CheatEffect::GrowInstant(_) => None, // instant, no duration
    };
    Self { effect, remaining }
  }

  pub fn effect(&self) -> &CheatEffect {
    &self.effect
  }

  /// Returns `true` if the cheat has expired.
  pub(crate) fn tick(&mut self, dt: f32) -> bool {
    if let Some(ref mut r) = self.remaining {
      *r -= dt;
      *r <= 0.0
    } else {
      true // instant effects expire immediately
    }
  }
}

/// Apply a cheat effect to the player worm.
pub(crate) fn apply(worm: &mut Worm, effect: &CheatEffect) {
  match effect {
    CheatEffect::Invincibility(d) => worm.apply_invincibility(*d),
    CheatEffect::SpeedMultiplier(m, _) => worm.apply_speed_multiplier(*m),
    CheatEffect::ScoreMultiplier(m, _) => worm.apply_score_multiplier(*m),
    CheatEffect::GrowInstant(n) => worm.grow(*n),
    CheatEffect::Noclip(d) => worm.apply_noclip(*d),
  }
}

/// Expire old cheats and clean up their effects.
pub(crate) fn expire_cheats(worm: &mut Worm, active: &mut Vec<ActiveCheat>, dt: f32) {
  active.retain_mut(|cheat| {
    let expired = cheat.tick(dt);
    if expired {
      match cheat.effect() {
        CheatEffect::SpeedMultiplier(_, _) => {
          worm.reset_speed_multiplier();
        }
        CheatEffect::ScoreMultiplier(_, _) => {
          worm.reset_score_multiplier();
        }
        _ => {}
      }
    }
    !expired
  });
}
