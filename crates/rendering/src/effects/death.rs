use bevy::prelude::*;

use crate::theme::GruvboxTheme;

/// A particle spawned when a worm dies — scatters outward then fades.
#[derive(Component)]
struct DeathParticle {
  velocity: Vec2,
  lifetime: f32,
  max_lifetime: f32,
}

/// Flash overlay on death.
#[derive(Component)]
struct DeathFlash {
  timer: f32,
}

pub(crate) struct DeathEffectPlugin;

impl Plugin for DeathEffectPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (tick_death_particles, tick_death_flash));
  }
}

/// Spawn death particles at a position. Called externally.
pub fn spawn_death_burst(commands: &mut Commands, position: Vec2, count: usize, color: Color) {
  use rand::Rng;
  let mut rng = rand::thread_rng();

  for _ in 0..count {
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let speed = rng.gen_range(80.0..250.0);
    let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
    let size = rng.gen_range(4.0..10.0);
    let lifetime = rng.gen_range(0.4..0.8);

    commands.spawn((
      Sprite {
        color,
        custom_size: Some(Vec2::splat(size)),
        ..default()
      },
      Transform::from_translation(position.extend(10.0)),
      DeathParticle {
        velocity,
        lifetime,
        max_lifetime: lifetime,
      },
    ));
  }
}

/// Spawn a red flash overlay.
pub fn spawn_death_flash(commands: &mut Commands, theme: &GruvboxTheme) {
  commands.spawn((
    Sprite {
      color: theme.red.with_alpha(0.3),
      custom_size: Some(Vec2::splat(5000.0)),
      ..default()
    },
    Transform::from_xyz(0.0, 0.0, 100.0),
    DeathFlash { timer: 0.25 },
  ));
}

fn tick_death_particles(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Transform, &mut DeathParticle, &mut Sprite)>,
  time: Res<Time>,
) {
  let dt = time.delta_secs();

  for (entity, mut transform, mut particle, mut sprite) in &mut query {
    particle.lifetime -= dt;
    if particle.lifetime <= 0.0 {
      commands.entity(entity).despawn();
      continue;
    }

    // Move
    transform.translation.x += particle.velocity.x * dt;
    transform.translation.y += particle.velocity.y * dt;

    // Slow down
    particle.velocity *= 0.95;

    // Fade out
    let alpha = particle.lifetime / particle.max_lifetime;
    sprite.color = sprite.color.with_alpha(alpha);

    // Shrink
    if let Some(ref mut size) = sprite.custom_size {
      *size *= 0.98;
    }
  }
}

fn tick_death_flash(mut commands: Commands, mut query: Query<(Entity, &mut DeathFlash, &mut Sprite)>, time: Res<Time>) {
  let dt = time.delta_secs();
  for (entity, mut flash, mut sprite) in &mut query {
    flash.timer -= dt;
    if flash.timer <= 0.0 {
      commands.entity(entity).despawn();
      continue;
    }
    let alpha = flash.timer / 0.25;
    sprite.color = sprite.color.with_alpha(alpha * 0.3);
  }
}
