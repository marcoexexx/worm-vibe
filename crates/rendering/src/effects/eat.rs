use bevy::prelude::*;

/// A small pop/scale effect when food is collected.
#[derive(Component)]
struct EatPop {
  timer: f32,
}

pub(crate) struct EatEffectPlugin;

impl Plugin for EatEffectPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, tick_eat_pops);
  }
}

/// Spawn a brief pop at food position.
pub fn spawn_eat_pop(commands: &mut Commands, position: Vec2, color: Color) {
  commands.spawn((
    Sprite {
      color: color.with_alpha(0.8),
      custom_size: Some(Vec2::splat(6.0)),
      ..default()
    },
    Transform::from_translation(position.extend(8.0)),
    EatPop { timer: 0.2 },
  ));
}

fn tick_eat_pops(
  mut commands: Commands,
  mut query: Query<(Entity, &mut EatPop, &mut Transform, &mut Sprite)>,
  time: Res<Time>,
) {
  let dt = time.delta_secs();
  for (entity, mut pop, mut transform, mut sprite) in &mut query {
    pop.timer -= dt;
    if pop.timer <= 0.0 {
      commands.entity(entity).despawn();
      continue;
    }

    // Scale up
    let progress = 1.0 - (pop.timer / 0.2);
    let scale = 1.0 + progress * 2.0;
    transform.scale = Vec3::splat(scale);

    // Fade out
    let alpha = pop.timer / 0.2;
    sprite.color = sprite.color.with_alpha(alpha * 0.8);
  }
}
