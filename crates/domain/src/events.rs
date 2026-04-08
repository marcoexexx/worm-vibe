use crate::{CheatEffect, FoodKind, WormId};
use glam::Vec2;

#[derive(Debug, Clone)]
pub enum DomainEvent {
  FoodEaten {
    worm_id: WormId,
    position: Vec2,
    kind: FoodKind,
    new_score: u64,
  },
  WormDied {
    worm_id: WormId,
    position: Vec2,
    final_score: u64,
    segments_dropped: usize,
  },
  WormGrew {
    worm_id: WormId,
    new_length: usize,
  },
  CheatActivated {
    effect: CheatEffect,
  },
  BoostStarted {
    worm_id: WormId,
  },
  BoostEnded {
    worm_id: WormId,
  },
}
