//! Integration tests for the game loop.
//! These test the full GameWorld lifecycle without Bevy ECS.

use app::{tick_game_world, GameWorld};
use domain::*;
use glam::Vec2;

fn easy_world() -> GameWorld {
  GameWorld::new(GameConfig::from_preset(DifficultyPreset::Easy))
}

// ============================================================================
// World initialization
// ============================================================================

#[test]
fn world_initializes_with_player_alive() {
  let world = easy_world();
  assert!(world.player().is_alive());
  assert_eq!(world.player().name(), "You");
  assert_eq!(world.player().head_position(), Vec2::ZERO);
}

#[test]
fn world_spawns_correct_ai_count() {
  let world = easy_world();
  let expected = world.config().arena().max_ai_worms();
  assert_eq!(world.ai_worms().len(), expected);
}

#[test]
fn world_spawns_food() {
  let world = easy_world();
  assert!(!world.foods().is_empty());
}

#[test]
fn world_starts_at_zero_elapsed() {
  let world = easy_world();
  assert!((world.elapsed() - 0.0).abs() < f32::EPSILON);
}

#[test]
fn player_has_spawn_protection() {
  let world = easy_world();
  assert!(world.player().is_invincible());
}

#[test]
fn all_ai_worms_alive_at_start() {
  let world = easy_world();
  for (worm, _) in world.ai_worms() {
    assert!(worm.is_alive());
  }
}

#[test]
fn ai_worms_not_at_center() {
  let world = easy_world();
  for (worm, _) in world.ai_worms() {
    let dist = worm.head_position().distance(Vec2::ZERO);
    assert!(dist > 100.0, "AI worm should spawn away from center, got dist={}", dist);
  }
}

// ============================================================================
// Ticking the game loop
// ============================================================================

#[test]
fn tick_advances_elapsed_time() {
  let mut world = easy_world();
  tick_game_world(&mut world, MovementIntent::idle(), 0.016);
  assert!(world.elapsed() > 0.0);
}

#[test]
fn tick_moves_player_forward() {
  let mut world = easy_world();
  let start = world.player().head_position();

  // Tick several frames to let the worm accelerate and move
  for _ in 0..10 {
    tick_game_world(&mut world, MovementIntent::idle(), 0.016);
  }

  let end = world.player().head_position();
  assert_ne!(start, end, "player should move forward");
}

#[test]
fn tick_with_direction_turns_player() {
  let mut world = easy_world();
  let intent = MovementIntent::new(Vec2::Y); // turn upward

  for _ in 0..30 {
    tick_game_world(&mut world, intent, 0.016);
  }

  // Player should be heading roughly upward
  let heading = world.player().heading();
  // heading should be closer to PI/2 than 0
  assert!(heading.abs() > 0.1, "player should have turned, heading={}", heading);
}

#[test]
fn multiple_ticks_dont_crash() {
  let mut world = easy_world();
  // Run 600 frames (~10 seconds) — should not panic
  for _ in 0..600 {
    tick_game_world(&mut world, MovementIntent::idle(), 0.016);
  }
  // Just verify it survived
  assert!(world.elapsed() > 9.0);
}

#[test]
fn food_count_stays_stable() {
  let mut world = easy_world();
  let _initial = world.foods().len();

  for _ in 0..100 {
    tick_game_world(&mut world, MovementIntent::idle(), 0.016);
  }

  // Food count should be roughly the same (eaten food gets replenished)
  let current = world.foods().len();
  let target = world.config().arena().target_food_count();
  assert!(
    current >= target / 2,
    "food should be replenished, got {} (target {})",
    current,
    target
  );
}

// ============================================================================
// Events
// ============================================================================

#[test]
fn tick_returns_events() {
  let mut world = easy_world();
  // Tick enough frames that something happens (AI eats food, etc.)
  let mut all_events = Vec::new();
  for _ in 0..300 {
    let events = tick_game_world(&mut world, MovementIntent::idle(), 0.016);
    all_events.extend(events);
  }
  // Over 300 frames, AI worms should eat some food
  let food_eaten = all_events
    .iter()
    .filter(|e| matches!(e, DomainEvent::FoodEaten { .. }))
    .count();
  assert!(food_eaten > 0, "expected some FoodEaten events over 300 frames");
}

// ============================================================================
// Boost mechanic
// ============================================================================

#[test]
fn boost_drains_segments() {
  let mut world = easy_world();
  // Grow the player first so they can boost
  world.activate_cheat(CheatEffect::GrowInstant(20));
  let before = world.player().length();

  let intent = MovementIntent::new(Vec2::X).with_boost();
  for _ in 0..120 {
    tick_game_world(&mut world, intent, 0.016);
  }

  assert!(
    world.player().length() < before,
    "boosting should drain segments: before={}, after={}",
    before,
    world.player().length()
  );
}

// ============================================================================
// Cheats
// ============================================================================

#[test]
fn cheat_grow_increases_length() {
  let mut world = easy_world();
  let before = world.player().length();
  world.activate_cheat(CheatEffect::GrowInstant(10));
  assert_eq!(world.player().length(), before + 10);
}

#[test]
fn cheat_invincibility_makes_player_invincible() {
  let mut world = easy_world();
  // Invincibility from spawn protection may already be active
  // Tick past it
  for _ in 0..200 {
    tick_game_world(&mut world, MovementIntent::idle(), 0.016);
  }
  assert!(!world.player().is_invincible(), "spawn protection should expire");

  world.activate_cheat(CheatEffect::Invincibility(5.0));
  assert!(world.player().is_invincible());
}

#[test]
fn cheat_noclip_activates() {
  let mut world = easy_world();
  world.activate_cheat(CheatEffect::Noclip(10.0));
  assert!(world.player().has_noclip());
}

// ============================================================================
// Difficulty presets
// ============================================================================

#[test]
fn all_presets_create_valid_worlds() {
  for preset in [DifficultyPreset::Easy, DifficultyPreset::Normal, DifficultyPreset::Hard] {
    let world = GameWorld::new(GameConfig::from_preset(preset));
    assert!(world.player().is_alive());
    assert!(!world.foods().is_empty());
    assert!(!world.ai_worms().is_empty());
  }
}

#[test]
fn harder_presets_have_more_ai() {
  let easy = GameWorld::new(GameConfig::from_preset(DifficultyPreset::Easy));
  let normal = GameWorld::new(GameConfig::from_preset(DifficultyPreset::Normal));
  let hard = GameWorld::new(GameConfig::from_preset(DifficultyPreset::Hard));

  assert!(normal.ai_worms().len() > easy.ai_worms().len());
  assert!(hard.ai_worms().len() > normal.ai_worms().len());
}
