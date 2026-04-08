use bevy::prelude::*;
use rand::Rng;

/// Resource controlling screen shake intensity.
#[derive(Resource, Default)]
pub struct ScreenShake {
  intensity: f32,
  duration: f32,
  timer: f32,
}

impl ScreenShake {
  pub fn trigger(&mut self, intensity: f32, duration: f32) {
    self.intensity = intensity;
    self.duration = duration;
    self.timer = duration;
  }
}

pub(crate) struct ScreenShakePlugin;

impl Plugin for ScreenShakePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<ScreenShake>()
      .add_systems(Update, apply_screen_shake);
  }
}

fn apply_screen_shake(
  mut shake: ResMut<ScreenShake>,
  mut cameras: Query<&mut Transform, With<Camera2d>>,
  time: Res<Time>,
) {
  let dt = time.delta_secs();

  if shake.timer <= 0.0 {
    return;
  }

  shake.timer -= dt;
  let progress = (shake.timer / shake.duration).max(0.0);
  let current_intensity = shake.intensity * progress;

  if current_intensity < 0.01 {
    return;
  }

  let mut rng = rand::thread_rng();
  let offset_x = rng.gen_range(-current_intensity..current_intensity);
  let offset_y = rng.gen_range(-current_intensity..current_intensity);

  for mut transform in &mut cameras {
    // Apply shake offset (gets overridden by camera follow next frame)
    transform.translation.x += offset_x;
    transform.translation.y += offset_y;
  }
}
