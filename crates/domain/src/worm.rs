use crate::WormConfig;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WormId(u64);

impl WormId {
  pub fn new(id: u64) -> Self {
    Self(id)
  }

  pub fn val(self) -> u64 {
    self.0
  }
}

#[derive(Debug, Clone)]
pub struct WormSegment {
  position: Vec2,
  radius: f32,
}

impl WormSegment {
  pub fn new(position: Vec2, radius: f32) -> Self {
    Self { position, radius }
  }

  pub fn position(&self) -> Vec2 {
    self.position
  }

  pub fn radius(&self) -> f32 {
    self.radius
  }

  pub fn set_position(&mut self, pos: Vec2) {
    self.position = pos;
  }
}

#[derive(Debug, Clone)]
pub struct Worm {
  id: WormId,
  name: String,
  segments: Vec<WormSegment>,
  heading: f32,
  current_speed: f32,
  target_speed: f32,
  score: u64,
  alive: bool,
  boosting: bool,
  boost_drain_timer: f32,
  invincible_timer: f32,
  speed_multiplier: f32,
  score_multiplier: f32,
  noclip_timer: f32,
  prev_head_position: Vec2,
  // Snapshot of config values needed at runtime
  cfg_base_speed: f32,
  cfg_max_speed: f32,
  cfg_acceleration: f32,
  cfg_deceleration: f32,
  cfg_boost_multiplier: f32,
  cfg_boost_drain_interval: f32,
  cfg_min_segments_for_boost: usize,
  cfg_length_speed_ln_factor: f32,
  cfg_length_speed_threshold: usize,
  cfg_base_radius: f32,
  cfg_radius_growth_factor: f32,
  cfg_max_radius_multiplier: f32,
}

impl Worm {
  pub fn spawn(id: WormId, position: Vec2, heading: f32, initial_length: usize, cfg: &WormConfig) -> Self {
    let dir = Vec2::new(heading.cos(), heading.sin());
    let mut segments = Vec::with_capacity(initial_length);
    for i in 0..initial_length {
      let offset = -dir * (i as f32) * cfg.segment_spawn_spacing;
      segments.push(WormSegment::new(position + offset, cfg.base_radius));
    }

    Self {
      id,
      name: String::new(),
      segments,
      heading,
      current_speed: cfg.base_speed * cfg.initial_speed_fraction,
      target_speed: cfg.base_speed,
      score: 0,
      alive: true,
      boosting: false,
      boost_drain_timer: 0.0,
      invincible_timer: 0.0,
      speed_multiplier: 1.0,
      score_multiplier: 1.0,
      noclip_timer: 0.0,
      prev_head_position: position,
      cfg_base_speed: cfg.base_speed,
      cfg_max_speed: cfg.max_speed,
      cfg_acceleration: cfg.acceleration,
      cfg_deceleration: cfg.deceleration,
      cfg_boost_multiplier: cfg.boost_multiplier,
      cfg_boost_drain_interval: cfg.boost_drain_interval,
      cfg_min_segments_for_boost: cfg.min_segments_for_boost,
      cfg_length_speed_ln_factor: cfg.length_speed_ln_factor,
      cfg_length_speed_threshold: cfg.length_speed_threshold,
      cfg_base_radius: cfg.base_radius,
      cfg_radius_growth_factor: cfg.radius_growth_factor,
      cfg_max_radius_multiplier: cfg.max_radius_multiplier,
    }
  }

  // --- Accessors ---

  pub fn id(&self) -> WormId {
    self.id
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn segments(&self) -> &[WormSegment] {
    &self.segments
  }

  pub fn head_position(&self) -> Vec2 {
    self.segments[0].position
  }

  pub fn prev_head_position(&self) -> Vec2 {
    self.prev_head_position
  }

  pub fn head_radius(&self) -> f32 {
    self.segments[0].radius
  }

  pub fn heading(&self) -> f32 {
    self.heading
  }

  pub fn current_speed(&self) -> f32 {
    self.current_speed
  }

  pub fn target_speed(&self) -> f32 {
    self.target_speed
  }

  pub fn score(&self) -> u64 {
    self.score
  }

  pub fn is_alive(&self) -> bool {
    self.alive
  }

  pub fn is_boosting(&self) -> bool {
    self.boosting
  }

  pub fn is_invincible(&self) -> bool {
    self.invincible_timer > 0.0
  }

  pub fn has_noclip(&self) -> bool {
    self.noclip_timer > 0.0
  }

  pub fn length(&self) -> usize {
    self.segments.len()
  }

  // --- Mutations ---

  pub fn set_heading(&mut self, heading: f32) {
    self.heading = heading;
  }

  pub fn set_head_position(&mut self, pos: Vec2) {
    self.prev_head_position = self.segments[0].position;
    self.segments[0].set_position(pos);
  }

  pub fn segment_mut(&mut self, index: usize) -> Option<&mut WormSegment> {
    self.segments.get_mut(index)
  }

  pub fn add_score(&mut self, points: u64) {
    self.score += (points as f32 * self.score_multiplier) as u64;
  }

  pub fn grow(&mut self, count: usize) {
    let tail_pos = self.segments.last().map_or(Vec2::ZERO, |s| s.position);
    let radius = self.current_radius();
    for _ in 0..count {
      self.segments.push(WormSegment::new(tail_pos, radius));
    }
    self.update_all_radii();
  }

  /// Current radius based on worm length.
  /// radius = base_radius * min(1 + growth_factor * ln(length), max_multiplier)
  pub fn current_radius(&self) -> f32 {
    let length = self.segments.len().max(1) as f32;
    let multiplier = (1.0 + self.cfg_radius_growth_factor * length.ln()).min(self.cfg_max_radius_multiplier);
    self.cfg_base_radius * multiplier.max(1.0)
  }

  /// Recalculate all segment radii (call after length changes).
  fn update_all_radii(&mut self) {
    let r = self.current_radius();
    for seg in &mut self.segments {
      seg.radius = r;
    }
  }

  pub fn kill(&mut self) {
    self.alive = false;
  }

  pub fn set_boosting(&mut self, boosting: bool) {
    if boosting && self.segments.len() < self.cfg_min_segments_for_boost {
      return;
    }
    self.boosting = boosting;
  }

  /// Tick acceleration: smoothly ramp current_speed toward target_speed.
  pub fn tick_acceleration(&mut self, dt: f32) {
    let bonus = self.length_speed_bonus();
    let base = self.cfg_base_speed + bonus;
    let boost = if self.boosting { self.cfg_boost_multiplier } else { 1.0 };
    self.target_speed = (base * self.speed_multiplier * boost).min(self.cfg_max_speed * self.cfg_boost_multiplier);

    if self.current_speed < self.target_speed {
      self.current_speed = (self.current_speed + self.cfg_acceleration * dt).min(self.target_speed);
    } else if self.current_speed > self.target_speed {
      self.current_speed = (self.current_speed - self.cfg_deceleration * dt).max(self.target_speed);
    }
  }

  pub fn tick_boost_drain(&mut self, dt: f32) -> bool {
    if !self.boosting {
      return false;
    }
    self.boost_drain_timer += dt;
    if self.boost_drain_timer >= self.cfg_boost_drain_interval {
      self.boost_drain_timer -= self.cfg_boost_drain_interval;
      if self.segments.len() > self.cfg_min_segments_for_boost {
        self.segments.pop();
        self.update_all_radii(); // shrink when losing segments
        return true;
      }
      self.boosting = false;
    }
    false
  }

  pub fn tick_timers(&mut self, dt: f32) {
    if self.invincible_timer > 0.0 {
      self.invincible_timer = (self.invincible_timer - dt).max(0.0);
    }
    if self.noclip_timer > 0.0 {
      self.noclip_timer = (self.noclip_timer - dt).max(0.0);
    }
  }

  // --- Cheat applications ---

  pub fn apply_invincibility(&mut self, duration: f32) {
    self.invincible_timer = duration;
  }

  pub fn apply_speed_multiplier(&mut self, multiplier: f32) {
    self.speed_multiplier = multiplier;
  }

  pub fn apply_score_multiplier(&mut self, multiplier: f32) {
    self.score_multiplier = multiplier;
  }

  pub fn apply_noclip(&mut self, duration: f32) {
    self.noclip_timer = duration;
  }

  pub fn reset_speed_multiplier(&mut self) {
    self.speed_multiplier = 1.0;
  }

  pub fn reset_score_multiplier(&mut self) {
    self.score_multiplier = 1.0;
  }

  // --- Private ---

  fn length_speed_bonus(&self) -> f32 {
    if self.segments.len() <= self.cfg_length_speed_threshold {
      return 0.0;
    }
    (self.segments.len() as f32).ln() * self.cfg_length_speed_ln_factor
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::WormConfig;

  fn default_cfg() -> WormConfig {
    WormConfig::default()
  }

  fn spawn_worm(length: usize) -> Worm {
    let cfg = default_cfg();
    Worm::spawn(WormId::new(1), Vec2::ZERO, 0.0, length, &cfg)
  }

  #[test]
  fn spawn_creates_correct_segment_count() {
    let worm = spawn_worm(5);
    assert_eq!(worm.length(), 5);
    assert!(worm.is_alive());
    assert_eq!(worm.score(), 0);
  }

  #[test]
  fn head_position_is_spawn_position() {
    let cfg = default_cfg();
    let pos = Vec2::new(100.0, 200.0);
    let worm = Worm::spawn(WormId::new(0), pos, 0.0, 3, &cfg);
    assert_eq!(worm.head_position(), pos);
  }

  #[test]
  fn segments_trail_behind_head() {
    let worm = spawn_worm(5);
    // Heading 0 = right, so segments trail to the left (negative x)
    for i in 1..worm.length() {
      assert!(
        worm.segments()[i].position().x <= worm.segments()[i - 1].position().x,
        "segment {} should be behind segment {}",
        i,
        i - 1
      );
    }
  }

  #[test]
  fn grow_adds_segments_at_tail() {
    let mut worm = spawn_worm(3);
    assert_eq!(worm.length(), 3);
    worm.grow(4);
    assert_eq!(worm.length(), 7);
  }

  #[test]
  fn add_score_accumulates() {
    let mut worm = spawn_worm(3);
    worm.add_score(10);
    worm.add_score(5);
    assert_eq!(worm.score(), 15);
  }

  #[test]
  fn score_multiplier_affects_score() {
    let mut worm = spawn_worm(3);
    worm.apply_score_multiplier(2.0);
    worm.add_score(10);
    assert_eq!(worm.score(), 20);
  }

  #[test]
  fn reset_score_multiplier_returns_to_normal() {
    let mut worm = spawn_worm(3);
    worm.apply_score_multiplier(3.0);
    worm.add_score(10);
    worm.reset_score_multiplier();
    worm.add_score(10);
    assert_eq!(worm.score(), 40); // 30 + 10
  }

  #[test]
  fn kill_marks_dead() {
    let mut worm = spawn_worm(3);
    assert!(worm.is_alive());
    worm.kill();
    assert!(!worm.is_alive());
  }

  #[test]
  fn boosting_requires_minimum_segments() {
    let mut worm = spawn_worm(3); // min_segments_for_boost = 5
    worm.set_boosting(true);
    assert!(!worm.is_boosting(), "too few segments to boost");

    let mut worm = spawn_worm(10);
    worm.set_boosting(true);
    assert!(worm.is_boosting());
  }

  #[test]
  fn boost_drain_removes_tail_segment() {
    let mut worm = spawn_worm(10);
    worm.set_boosting(true);
    let initial = worm.length();

    // Tick enough to trigger one drain (interval = 0.4s)
    let drained = worm.tick_boost_drain(0.5);
    assert!(drained);
    assert_eq!(worm.length(), initial - 1);
  }

  #[test]
  fn boost_stops_at_minimum_segments() {
    let cfg = default_cfg();
    let min = cfg.min_segments_for_boost;
    let mut worm = spawn_worm(min + 1);
    worm.set_boosting(true);

    // Drain until it stops
    for _ in 0..20 {
      worm.tick_boost_drain(0.5);
    }
    assert!(!worm.is_boosting(), "should stop boosting at minimum segments");
  }

  #[test]
  fn invincibility_timer_decreases() {
    let mut worm = spawn_worm(3);
    worm.apply_invincibility(5.0);
    assert!(worm.is_invincible());

    worm.tick_timers(3.0);
    assert!(worm.is_invincible());

    worm.tick_timers(3.0);
    assert!(!worm.is_invincible(), "timer should have expired");
  }

  #[test]
  fn noclip_timer_decreases() {
    let mut worm = spawn_worm(3);
    worm.apply_noclip(2.0);
    assert!(worm.has_noclip());

    worm.tick_timers(3.0);
    assert!(!worm.has_noclip());
  }

  #[test]
  fn set_heading_updates_heading() {
    let mut worm = spawn_worm(3);
    worm.set_heading(1.5);
    assert!((worm.heading() - 1.5).abs() < f32::EPSILON);
  }

  #[test]
  fn set_head_position_tracks_previous() {
    let mut worm = spawn_worm(3);
    let old = worm.head_position();
    worm.set_head_position(Vec2::new(50.0, 50.0));
    assert_eq!(worm.prev_head_position(), old);
    assert_eq!(worm.head_position(), Vec2::new(50.0, 50.0));
  }

  #[test]
  fn tick_acceleration_ramps_speed() {
    let mut worm = spawn_worm(3);
    let initial_speed = worm.current_speed();
    // target_speed = base_speed, initial = base_speed * 0.5
    assert!(initial_speed < worm.target_speed());

    worm.tick_acceleration(1.0);
    assert!(worm.current_speed() > initial_speed);
  }

  #[test]
  fn worm_id_round_trips() {
    let id = WormId::new(42);
    assert_eq!(id.val(), 42);
  }

  #[test]
  fn name_set_and_get() {
    let mut worm = spawn_worm(3);
    assert_eq!(worm.name(), "");
    worm.set_name("TestBot".to_string());
    assert_eq!(worm.name(), "TestBot");
  }

  #[test]
  fn radius_grows_with_length() {
    let mut worm = spawn_worm(3);
    let small_radius = worm.current_radius();

    // Grow to 50 segments
    worm.grow(47);
    let big_radius = worm.current_radius();

    assert!(
      big_radius > small_radius,
      "longer worm should be wider: {} vs {}",
      big_radius,
      small_radius
    );
    // All segments should have the same radius
    for seg in worm.segments() {
      assert!((seg.radius() - big_radius).abs() < 0.01);
    }
  }

  #[test]
  fn radius_capped_at_max_multiplier() {
    let mut worm = spawn_worm(3);
    let cfg = default_cfg();

    // Grow to a huge size
    worm.grow(500);
    let radius = worm.current_radius();
    let max = cfg.base_radius * cfg.max_radius_multiplier;

    assert!(radius <= max + 0.01, "radius {} should not exceed max {}", radius, max);
  }

  #[test]
  fn radius_shrinks_when_boosting() {
    let mut worm = spawn_worm(20);
    let before = worm.current_radius();
    worm.set_boosting(true);

    // Drain several segments
    for _ in 0..10 {
      worm.tick_boost_drain(0.5);
    }
    let after = worm.current_radius();

    assert!(
      after < before || worm.length() < 20,
      "radius should shrink or length decrease after boost drain"
    );
  }
}
