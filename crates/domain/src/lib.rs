mod arena;
mod cheats;
mod config;
mod direction;
mod events;
mod food;
mod score;
mod worm;

pub use arena::{ArenaBounds, ArenaConfig};
pub use cheats::{CheatCode, CheatEffect, CheatRegistry};
pub use config::{
  AiDifficulty, AiTuning, CheatDefaults, CollisionConfig, DifficultyPreset, FoodConfig, GameConfig, WormConfig,
};
pub use direction::{Heading, MovementIntent};
pub use events::DomainEvent;
pub use food::{Food, FoodKind};
pub use score::{ScoreHistory, ScorePersistence, ScoreRecord};
pub use worm::{Worm, WormId, WormSegment};
