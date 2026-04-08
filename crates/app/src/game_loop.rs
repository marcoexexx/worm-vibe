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
    // Spawn protection: brief invincibility so the player can't die instantly
    player.apply_invincibility(2.0);

    let half = config.arena().half_extents();
    let ai_count = config.arena().max_ai_worms();
    let mut ai_worms = Vec::with_capacity(ai_count);
    for i in 1..=ai_count {
      let id = WormId::new(i as u64);
      let worm = worm_service::spawn_at_random_edge(
        id,
        half,
        config.initial_worm_length(),
        config.worm(),
        config.collision(),
        &mut rng,
      );
      let difficulty = config.ai_difficulty_for(i - 1);
      let mut worm = worm;
      worm.set_name(random_bot_name(&mut rng));
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

  // --- Public accessors ---

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

  /// Activate a cheat by effect.
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

/// Advance the game world by one frame. Returns domain events.
pub fn tick_game_world(world: &mut GameWorld, intent: MovementIntent, dt: f32) -> Vec<DomainEvent> {
  let mut events = Vec::new();
  world.elapsed += dt;

  // --- Timers & cheats ---
  world.player.tick_timers(dt);
  cheat_service::expire_cheats(&mut world.player, &mut world.active_cheats, dt);

  // --- Player movement ---
  let turn_rate = world.config.turn_rate();
  let segment_spacing = world.config.worm().segment_follow_spacing;
  let follow_speed = world.config.worm().follow_speed;
  move_worm(
    &mut world.player,
    intent.target_heading(),
    turn_rate,
    segment_spacing,
    follow_speed,
    dt,
  );

  // --- Player boost ---
  world.player.set_boosting(intent.wants_boost());
  if world.player.tick_boost_drain(dt) {
    events.push(DomainEvent::BoostStarted {
      worm_id: world.player.id(),
    });
  }

  // --- AI movement ---
  tick_ai_worms(world, dt);

  // --- Collision detection ---
  let bounds = ArenaBounds::from(world.config.arena());
  let results = collision_service::detect_collisions(
    &world.player,
    &world.ai_worms.iter().map(|(w, _)| w.clone()).collect::<Vec<_>>(),
    &world.foods,
    &bounds,
    world.config.collision(),
  );

  // Process results
  let mut eaten_food_indices = Vec::new();
  let mut dead_worm_ids = Vec::new();

  for result in results {
    match result {
      CollisionResult::WormAteFood { worm_id, food_index } => {
        if eaten_food_indices.contains(&food_index) {
          continue;
        }
        if let Some(food) = world.foods.get(food_index) {
          let kind = food.kind();
          let pos = food.position();

          if let Some(worm) = find_worm_mut(world, worm_id) {
            worm.add_score(kind.score_value());
            worm.grow(kind.growth());
            events.push(DomainEvent::FoodEaten {
              worm_id,
              position: pos,
              kind,
              new_score: worm.score(),
            });
          }
          eaten_food_indices.push(food_index);
        }
      }
      CollisionResult::WormHitWorm { victim, killer: _ } => {
        if let Some(worm) = find_worm_ref(world, victim) {
          if worm.is_invincible() {
            continue;
          }
        }
        if !dead_worm_ids.contains(&victim) {
          dead_worm_ids.push(victim);
          if let Some(worm) = find_worm_ref(world, victim) {
            events.push(DomainEvent::WormDied {
              worm_id: victim,
              position: worm.head_position(),
              final_score: worm.score(),
              segments_dropped: worm.length(),
            });
          }
        }
      }
      CollisionResult::WormHitBoundary { worm_id } => {
        if let Some(worm) = find_worm_ref(world, worm_id) {
          if worm.is_invincible() {
            continue;
          }
        }
        if !dead_worm_ids.contains(&worm_id) {
          dead_worm_ids.push(worm_id);
          if let Some(worm) = find_worm_ref(world, worm_id) {
            events.push(DomainEvent::WormDied {
              worm_id,
              position: worm.head_position(),
              final_score: worm.score(),
              segments_dropped: worm.length(),
            });
          }
        }
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

  // Kill dead worms, drop food from their segments
  for worm_id in &dead_worm_ids {
    let (positions, worm_radius) = collect_segment_info(world, *worm_id);
    let dropped = food_service::drop_from_segments(&positions, worm_radius, world.config.food(), &mut world.rng);
    world.foods.extend(dropped);
  }

  for worm_id in &dead_worm_ids {
    kill_worm(world, *worm_id);
  }

  // Food magnet: move food slightly toward worm heads when within 2x food radius
  apply_food_magnet(world, dt);

  // Respawn AI worms (ensure minimum distance from player)
  let half = world.config.arena().half_extents();
  let target_ai = world.config.arena().max_ai_worms();
  let player_pos = world.player.head_position();
  let min_spawn_dist = 400.0; // minimum distance from player head
  while world.ai_worms.len() < target_ai {
    let idx = world.ai_worms.len();
    let id = world.allocate_worm_id();
    let mut worm = worm_service::spawn_at_random_edge(
      id,
      half,
      world.config.initial_worm_length(),
      world.config.worm(),
      world.config.collision(),
      &mut world.rng,
    );
    // Re-roll position if too close to player (max 5 attempts)
    for _ in 0..5 {
      if worm.head_position().distance(player_pos) >= min_spawn_dist {
        break;
      }
      worm = worm_service::spawn_at_random_edge(
        id,
        half,
        world.config.initial_worm_length(),
        world.config.worm(),
        world.config.collision(),
        &mut world.rng,
      );
    }
    let difficulty = world.config.ai_difficulty_for(idx);
    worm.set_name(random_bot_name(&mut world.rng));
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

  events
}

// --- Private helpers ---

fn move_worm(
  worm: &mut Worm,
  target_heading: Option<f32>,
  turn_rate: f32,
  segment_spacing: f32,
  follow_spd: f32,
  dt: f32,
) {
  // Smooth turning — scale turn rate with length so longer worms turn wider.
  // Minimum turning radius must exceed the worm body width to prevent self-overlap.
  // effective_rate = base_rate / (1 + 0.15 * ln(length))
  if let Some(target) = target_heading {
    let length = worm.length().max(1) as f32;
    let effective_turn_rate = turn_rate / (1.0 + 0.15 * length.ln());
    let new_heading = smooth_turn(worm.heading(), target, effective_turn_rate, dt);
    worm.set_heading(new_heading);
  }

  // Acceleration: ramp speed toward target
  worm.tick_acceleration(dt);

  // Move head at current (interpolated) speed
  let new_head = move_head(worm.head_position(), worm.heading(), worm.current_speed(), dt);
  worm.set_head_position(new_head);

  // Follow-the-leader for body segments (smooth curves)
  let len = worm.segments().len();
  for i in 1..len {
    let leader_pos = worm.segments()[i - 1].position();
    let follower_pos = worm.segments()[i].position();
    let new_pos = follow_segment(leader_pos, follower_pos, segment_spacing, follow_spd, dt);
    if let Some(seg) = worm.segment_mut(i) {
      seg.set_position(new_pos);
    }
  }
}

fn tick_ai_worms(world: &mut GameWorld, dt: f32) {
  let half = world.config.arena().half_extents();
  let turn_rate = world.config.turn_rate();
  let segment_spacing = world.config.worm().segment_follow_spacing;
  let follow_spd = world.config.worm().follow_speed;

  for i in 0..world.ai_worms.len() {
    world.ai_worms[i].1.update(dt);

    // Build perception with per-worm radius
    let (worm, brain) = &world.ai_worms[i];
    let perception_radius = brain.perception_radius();
    let perception = build_perception(
      worm,
      &world.foods,
      &world.ai_worms,
      &world.player,
      half,
      perception_radius,
    );

    let decisions = {
      let mut local_rng = StdRng::from_seed(world.rng.gen());
      let (_, brain) = &mut world.ai_worms[i];
      brain.decide(&perception, &mut local_rng)
    };

    let mut boost = false;
    let mut target = None;
    for d in &decisions {
      match d {
        AiDecision::TurnTo(h) => target = Some(*h),
        AiDecision::Boost => boost = true,
      }
    }

    let (worm, _) = &mut world.ai_worms[i];
    worm.set_boosting(boost);
    worm.tick_boost_drain(dt);
    move_worm(worm, target, turn_rate, segment_spacing, follow_spd, dt);
  }
}

/// Move food items that are within 2x food radius of any worm head toward that head.
fn apply_food_magnet(world: &mut GameWorld, dt: f32) {
  let food_cfg = world.config.food();
  let magnet_range = food_cfg.radius * food_cfg.magnet_range_multiplier;
  let pull_speed = food_cfg.magnet_pull_speed;

  // Collect all worm head positions
  let mut heads: Vec<Vec2> = Vec::new();
  if world.player.is_alive() {
    heads.push(world.player.head_position());
  }
  for (w, _) in &world.ai_worms {
    if w.is_alive() {
      heads.push(w.head_position());
    }
  }

  for food in &mut world.foods {
    let food_pos = food.position();
    for &head in &heads {
      let dist = food_pos.distance(head);
      if dist > 0.0 && dist < magnet_range {
        let dir = (head - food_pos).normalize();
        let step = (pull_speed * dt).min(dist);
        food.set_position(food_pos + dir * step);
        break; // pulled by the nearest head; one head wins
      }
    }
  }
}

fn build_perception(
  worm: &Worm,
  foods: &[Food],
  ai_worms: &[(Worm, BasicAiBrain)],
  player: &Worm,
  half_extents: Vec2,
  perception_radius: f32,
) -> AiPerception {
  let pos = worm.head_position();

  let nearby_food: Vec<_> = foods
    .iter()
    .filter(|f| f.position().distance(pos) < perception_radius)
    .map(|f| (f.position(), f.kind()))
    .collect();

  let mut nearby_worms = Vec::new();

  // Include player
  if player.is_alive() && player.head_position().distance(pos) < perception_radius {
    nearby_worms.push(ai::NearbyWorm {
      position: player.head_position(),
      length: player.length(),
      heading: player.heading(),
    });
  }

  // Include other AI worms
  for (other, _) in ai_worms {
    if other.id() == worm.id() || !other.is_alive() {
      continue;
    }
    if other.head_position().distance(pos) < perception_radius {
      nearby_worms.push(ai::NearbyWorm {
        position: other.head_position(),
        length: other.length(),
        heading: other.heading(),
      });
    }
  }

  AiPerception {
    self_position: pos,
    self_heading: worm.heading(),
    self_length: worm.length(),
    nearby_food,
    nearby_worms,
    arena_half_extents: half_extents,
  }
}

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

fn collect_segment_info(world: &GameWorld, id: WormId) -> (Vec<Vec2>, f32) {
  if let Some(worm) = find_worm_ref(world, id) {
    let positions = worm.segments().iter().map(|s| s.position()).collect();
    let radius = worm.current_radius();
    (positions, radius)
  } else {
    (Vec::new(), 11.0) // fallback to default food radius
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
  let prefix = PREFIXES[rng.gen_range(0..PREFIXES.len())];
  let suffix = SUFFIXES[rng.gen_range(0..SUFFIXES.len())];
  format!("{prefix}{suffix}")
}
