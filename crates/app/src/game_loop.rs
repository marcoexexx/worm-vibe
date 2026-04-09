use ai::{AiBrain, AiDecision, AiPerception, BasicAiBrain};
use bevy::prelude::Resource;
use domain::*;
use glam::Vec2;
use physics::{follow_segment, move_head, smooth_turn, CollisionResult};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::cheat_service::{self, ActiveCheat};
use crate::collision_service;
use crate::food_service;
use crate::worm_service;

// ============================================================================
// GameWorld
// ============================================================================

#[derive(Resource)]
pub struct GameWorld {
  config: GameConfig,
  player: Worm,
  ai_worms: Vec<(Worm, BasicAiBrain)>,
  foods: Vec<Food>,
  active_cheats: Vec<ActiveCheat>,
  next_worm_id: u64,
  rng: StdRng,
  elapsed: f32,
}

impl GameWorld {
  pub fn new(config: GameConfig) -> Self {
    let mut rng = StdRng::from_entropy();
    let player_id = WormId::new(0);
    let mut player = Worm::spawn(player_id, Vec2::ZERO, 0.0, config.initial_worm_length(), config.worm());
    player.set_name("You".to_string());
    player.apply_invincibility(2.0);

    let half = config.arena().half_extents();
    let ai_count = config.arena().max_ai_worms();
    let mut ai_worms = Vec::with_capacity(ai_count);
    for i in 1..=ai_count {
      let id = WormId::new(i as u64);
      let mut worm = spawn_ai_worm(id, half, &config, &mut rng);
      worm.set_name(random_bot_name(&mut rng));

      // Scale body size by rank so arena feels alive on spawn
      let rank_fraction = 1.0 - (i as f32 / ai_count as f32);
      let extra_segments = (rank_fraction * 200.0 + 20.0) as usize;
      let extra_score = extra_segments as u64 * 5;
      worm.grow(extra_segments);
      worm.add_score(extra_score);

      let difficulty = config.ai_difficulty_for(i - 1);
      ai_worms.push((worm, BasicAiBrain::with_difficulty(difficulty)));
    }

    let target = config.arena().target_food_count();
    let foods = food_service::spawn_batch(half, target, config.food(), &mut rng);

    Self {
      config,
      player,
      ai_worms,
      foods,
      active_cheats: Vec::new(),
      next_worm_id: (ai_count + 1) as u64,
      rng,
      elapsed: 0.0,
    }
  }

  pub fn player(&self) -> &Worm {
    &self.player
  }
  pub fn ai_worms(&self) -> &[(Worm, BasicAiBrain)] {
    &self.ai_worms
  }
  pub fn foods(&self) -> &[Food] {
    &self.foods
  }
  pub fn config(&self) -> &GameConfig {
    &self.config
  }
  pub fn elapsed(&self) -> f32 {
    self.elapsed
  }
  pub fn active_cheats(&self) -> &[ActiveCheat] {
    &self.active_cheats
  }

  pub fn activate_cheat(&mut self, effect: CheatEffect) {
    cheat_service::apply(&mut self.player, &effect);
    self.active_cheats.push(ActiveCheat::new(effect));
  }

  fn allocate_worm_id(&mut self) -> WormId {
    let id = WormId::new(self.next_worm_id);
    self.next_worm_id += 1;
    id
  }
}

// ============================================================================
// Game tick — main orchestrator
// ============================================================================

/// Advance the game world by one frame. Returns domain events.
pub fn tick_game_world(world: &mut GameWorld, intent: MovementIntent, dt: f32) -> Vec<DomainEvent> {
  let mut events = Vec::new();
  world.elapsed += dt;

  // Phase 1: Timers & cheats
  world.player.tick_timers(dt);
  cheat_service::expire_cheats(&mut world.player, &mut world.active_cheats, dt);

  // Phase 2: Movement (player + AI)
  tick_player_movement(world, &intent, dt, &mut events);
  tick_ai_worms(world, dt);

  // Phase 3: Collision detection & processing
  process_collisions(world, &mut events);

  // Phase 4: Food magnet
  apply_food_magnet(world, dt);

  // Phase 5: Respawn & replenish
  respawn_and_replenish(world);

  events
}

// ============================================================================
// Phase helpers
// ============================================================================

fn tick_player_movement(world: &mut GameWorld, intent: &MovementIntent, dt: f32, events: &mut Vec<DomainEvent>) {
  let turn_rate = world.config.turn_rate();
  let spacing = world.config.worm().segment_follow_spacing;
  let follow_spd = world.config.worm().follow_speed;

  move_worm(
    &mut world.player,
    intent.target_heading(),
    turn_rate,
    spacing,
    follow_spd,
    dt,
  );

  world.player.set_boosting(intent.wants_boost());
  if world.player.tick_boost_drain(dt) {
    events.push(DomainEvent::BoostStarted {
      worm_id: world.player.id(),
    });
  }
}

fn process_collisions(world: &mut GameWorld, events: &mut Vec<DomainEvent>) {
  let bounds = ArenaBounds::from(world.config.arena());
  let results = collision_service::detect_collisions(
    &world.player,
    &world.ai_worms,
    &world.foods,
    &bounds,
    world.config.collision(),
  );

  let mut eaten_food_indices = Vec::new();
  let mut dead_worm_ids = Vec::new();

  for result in results {
    match result {
      CollisionResult::WormAteFood { worm_id, food_index } => {
        process_food_eaten(world, worm_id, food_index, &mut eaten_food_indices, events);
      }
      CollisionResult::WormHitWorm { victim, .. } => {
        try_register_death(world, victim, &mut dead_worm_ids, events);
      }
      CollisionResult::WormHitBoundary { worm_id } => {
        try_register_death(world, worm_id, &mut dead_worm_ids, events);
      }
    }
  }

  // Remove eaten food (reverse order to preserve indices)
  eaten_food_indices.sort_unstable();
  eaten_food_indices.dedup();
  for idx in eaten_food_indices.into_iter().rev() {
    if idx < world.foods.len() {
      world.foods.swap_remove(idx);
    }
  }

  // Drop food from dead worms (score distributed across dropped food), then kill them
  for worm_id in &dead_worm_ids {
    let (positions, radius, score) = collect_segment_info(world, *worm_id);
    let dropped = food_service::drop_from_segments(&positions, radius, score, world.config.food(), &mut world.rng);
    world.foods.extend(dropped);
  }
  for worm_id in &dead_worm_ids {
    kill_worm(world, *worm_id);
  }
}

fn respawn_and_replenish(world: &mut GameWorld) {
  let half = world.config.arena().half_extents();
  let player_pos = world.player.head_position();
  let min_spawn_dist = 400.0;

  // Respawn dead AI worms
  let target_ai = world.config.arena().max_ai_worms();
  while world.ai_worms.len() < target_ai {
    let idx = world.ai_worms.len();
    let id = world.allocate_worm_id();
    let mut worm = spawn_ai_worm_safe(id, half, player_pos, min_spawn_dist, &world.config, &mut world.rng);
    worm.set_name(random_bot_name(&mut world.rng));
    let difficulty = world.config.ai_difficulty_for(idx);
    world.ai_worms.push((worm, BasicAiBrain::with_difficulty(difficulty)));
  }

  // Replenish food
  let target = world.config.arena().target_food_count();
  let max_spawn = world.config.food().max_spawn_per_tick;
  if world.foods.len() < target {
    let deficit = (target - world.foods.len()).min(max_spawn);
    let batch = food_service::spawn_batch(half, deficit, world.config.food(), &mut world.rng);
    world.foods.extend(batch);
  }
}

// ============================================================================
// Collision result helpers
// ============================================================================

fn process_food_eaten(
  world: &mut GameWorld,
  worm_id: WormId,
  food_index: usize,
  eaten: &mut Vec<usize>,
  events: &mut Vec<DomainEvent>,
) {
  if eaten.contains(&food_index) {
    return;
  }
  let Some(food) = world.foods.get(food_index) else {
    return;
  };
  let kind = food.kind();
  let pos = food.position();

  let total_score = food.total_score();
  let growth = kind.growth();

  if let Some(worm) = find_worm_mut(world, worm_id) {
    worm.add_score(total_score);
    worm.grow(growth);
    events.push(DomainEvent::FoodEaten {
      worm_id,
      position: pos,
      kind,
      new_score: worm.score(),
    });
  }
  eaten.push(food_index);
}

fn try_register_death(world: &GameWorld, worm_id: WormId, dead: &mut Vec<WormId>, events: &mut Vec<DomainEvent>) {
  if dead.contains(&worm_id) {
    return;
  }
  if let Some(worm) = find_worm_ref(world, worm_id) {
    if worm.is_invincible() {
      return;
    }
    dead.push(worm_id);
    events.push(DomainEvent::WormDied {
      worm_id,
      position: worm.head_position(),
      final_score: worm.score(),
      segments_dropped: worm.length(),
    });
  }
}

// ============================================================================
// Worm movement
// ============================================================================

fn move_worm(worm: &mut Worm, target_heading: Option<f32>, turn_rate: f32, spacing: f32, follow_spd: f32, dt: f32) {
  // Length-scaled turning: longer worms turn wider
  if let Some(target) = target_heading {
    let length = worm.length().max(1) as f32;
    let effective_rate = turn_rate / (1.0 + 0.15 * length.ln());
    worm.set_heading(smooth_turn(worm.heading(), target, effective_rate, dt));
  }

  worm.tick_acceleration(dt);

  let new_head = move_head(worm.head_position(), worm.heading(), worm.current_speed(), dt);
  worm.set_head_position(new_head);

  // Follow-the-leader for body segments
  for i in 1..worm.segments().len() {
    let leader = worm.segments()[i - 1].position();
    let follower = worm.segments()[i].position();
    let new_pos = follow_segment(leader, follower, spacing, follow_spd, dt);
    if let Some(seg) = worm.segment_mut(i) {
      seg.set_position(new_pos);
    }
  }
}

// ============================================================================
// AI
// ============================================================================

fn tick_ai_worms(world: &mut GameWorld, dt: f32) {
  let half = world.config.arena().half_extents();
  let turn_rate = world.config.turn_rate();
  let spacing = world.config.worm().segment_follow_spacing;
  let follow_spd = world.config.worm().follow_speed;

  for i in 0..world.ai_worms.len() {
    world.ai_worms[i].1.update(dt);

    let (worm, brain) = &world.ai_worms[i];
    let perception = build_perception(
      worm,
      &world.foods,
      &world.ai_worms,
      &world.player,
      half,
      brain.perception_radius(),
    );

    let decisions = {
      let mut local_rng = StdRng::from_seed(world.rng.gen());
      world.ai_worms[i].1.decide(&perception, &mut local_rng)
    };

    let (mut boost, mut target) = (false, None);
    for d in &decisions {
      match d {
        AiDecision::TurnTo(h) => target = Some(*h),
        AiDecision::Boost => boost = true,
      }
    }

    let (worm, _) = &mut world.ai_worms[i];
    worm.set_boosting(boost);
    worm.tick_boost_drain(dt);
    move_worm(worm, target, turn_rate, spacing, follow_spd, dt);
  }
}

fn build_perception(
  worm: &Worm,
  foods: &[Food],
  ai_worms: &[(Worm, BasicAiBrain)],
  player: &Worm,
  half_extents: Vec2,
  radius: f32,
) -> AiPerception {
  let pos = worm.head_position();
  let radius_sq = radius * radius;

  let nearby_food: Vec<_> = foods
    .iter()
    .filter(|f| f.position().distance_squared(pos) < radius_sq)
    .map(|f| (f.position(), f.kind()))
    .collect();

  let mut nearby_worms = Vec::new();
  if player.is_alive() && player.head_position().distance_squared(pos) < radius_sq {
    nearby_worms.push(ai::NearbyWorm {
      position: player.head_position(),
      length: player.length(),
      heading: player.heading(),
    });
  }
  for (other, _) in ai_worms {
    if other.id() == worm.id() || !other.is_alive() {
      continue;
    }
    if other.head_position().distance_squared(pos) < radius_sq {
      nearby_worms.push(ai::NearbyWorm {
        position: other.head_position(),
        length: other.length(),
        heading: other.heading(),
      });
    }
  }

  // Collect nearby body segments from other worms (short range for collision avoidance)
  let seg_range_sq = 80.0 * 80.0;
  let mut nearby_segments = Vec::new();
  if player.is_alive() && player.id() != worm.id() {
    for seg in player.segments().iter().skip(3) {
      if seg.position().distance_squared(pos) < seg_range_sq {
        nearby_segments.push(seg.position());
      }
    }
  }
  for (other, _) in ai_worms {
    if other.id() == worm.id() || !other.is_alive() {
      continue;
    }
    for seg in other.segments().iter().skip(3) {
      if seg.position().distance_squared(pos) < seg_range_sq {
        nearby_segments.push(seg.position());
      }
    }
  }

  AiPerception {
    self_position: pos,
    self_heading: worm.heading(),
    self_length: worm.length(),
    nearby_food,
    nearby_worms,
    nearby_segments,
    arena_half_extents: half_extents,
  }
}

// ============================================================================
// Food magnet
// ============================================================================

fn apply_food_magnet(world: &mut GameWorld, dt: f32) {
  let cfg = world.config.food();
  let magnet_range = cfg.radius * cfg.magnet_range_multiplier;
  let pull_speed = cfg.magnet_pull_speed;

  let mut heads: Vec<Vec2> = Vec::with_capacity(world.ai_worms.len() + 1);
  if world.player.is_alive() {
    heads.push(world.player.head_position());
  }
  for (w, _) in &world.ai_worms {
    if w.is_alive() {
      heads.push(w.head_position());
    }
  }

  for food in &mut world.foods {
    let fpos = food.position();
    for &head in &heads {
      let dist = fpos.distance(head);
      if dist > 0.0 && dist < magnet_range {
        let step = (pull_speed * dt).min(dist);
        food.set_position(fpos + (head - fpos).normalize() * step);
        break;
      }
    }
  }
}

// ============================================================================
// Spawn helpers
// ============================================================================

fn spawn_ai_worm(id: WormId, half: Vec2, config: &GameConfig, rng: &mut impl Rng) -> Worm {
  worm_service::spawn_at_random_edge(
    id,
    half,
    config.initial_worm_length(),
    config.worm(),
    config.collision(),
    rng,
  )
}

fn spawn_ai_worm_safe(
  id: WormId,
  half: Vec2,
  player_pos: Vec2,
  min_dist: f32,
  config: &GameConfig,
  rng: &mut impl Rng,
) -> Worm {
  let mut worm = spawn_ai_worm(id, half, config, rng);
  for _ in 0..5 {
    if worm.head_position().distance(player_pos) >= min_dist {
      break;
    }
    worm = spawn_ai_worm(id, half, config, rng);
  }
  worm
}

// ============================================================================
// Worm lookup helpers
// ============================================================================

fn find_worm_mut(world: &mut GameWorld, id: WormId) -> Option<&mut Worm> {
  if world.player.id() == id {
    return Some(&mut world.player);
  }
  world.ai_worms.iter_mut().find(|(w, _)| w.id() == id).map(|(w, _)| w)
}

fn find_worm_ref(world: &GameWorld, id: WormId) -> Option<&Worm> {
  if world.player.id() == id {
    return Some(&world.player);
  }
  world.ai_worms.iter().find(|(w, _)| w.id() == id).map(|(w, _)| w)
}

fn collect_segment_info(world: &GameWorld, id: WormId) -> (Vec<Vec2>, f32, u64) {
  if let Some(worm) = find_worm_ref(world, id) {
    (
      worm.segments().iter().map(|s| s.position()).collect(),
      worm.current_radius(),
      worm.score(),
    )
  } else {
    (Vec::new(), 11.0, 0)
  }
}

fn kill_worm(world: &mut GameWorld, id: WormId) {
  if world.player.id() == id {
    world.player.kill();
  } else {
    world.ai_worms.retain(|(w, _)| w.id() != id);
  }
}

fn random_bot_name(rng: &mut impl Rng) -> String {
  const PREFIXES: &[&str] = &[
    "Snek", "Worm", "Noodle", "Danger", "Slither", "Void", "Dark", "Pixel", "Cyber", "Glitch", "Turbo", "Mega",
    "Ultra", "Neo", "Zero", "Byte", "Bug", "Code", "Dev", "Sudo", "Root", "Hack", "Null", "Panic",
  ];
  const SUFFIXES: &[&str] = &[
    "3000", "Jr", "X", "Pro", "Bot", "AI", "99", "420", "7", "42", "Lord", "King", "Master", "Ninja", "Guru", "Noob",
    "Chad", "XD", "_dev", ".exe", ".rs", "++", "V2", "Max", "Lite", "HD",
  ];
  format!(
    "{}{}",
    PREFIXES[rng.gen_range(0..PREFIXES.len())],
    SUFFIXES[rng.gen_range(0..SUFFIXES.len())]
  )
}
