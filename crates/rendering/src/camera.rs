use app::GameWorld;
use bevy::prelude::*;

const LERP_SPEED: f32 = 5.0;

#[derive(Component)]
struct PlayerCamera;

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_camera)
      .add_systems(Update, follow_player);
  }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((Camera2d, PlayerCamera));
}

fn follow_player(world: Option<Res<GameWorld>>, mut query: Query<&mut Transform, With<PlayerCamera>>, time: Res<Time>) {
  let Some(world) = world else { return };
  let Ok(mut transform) = query.get_single_mut() else {
    return;
  };

  let target = world.player().head_position();
  let current = transform.translation.truncate();
  let dt = time.delta_secs();

  let new_pos = current.lerp(target, LERP_SPEED * dt);
  transform.translation.x = new_pos.x;
  transform.translation.y = new_pos.y;

  // Zoom out as worm grows
  let length = world.player().length() as f32;
  let target_scale = 1.0 + (length.ln().max(0.0)) * 0.1;
  let target_scale = target_scale.clamp(1.0, 2.5);
  transform.scale = transform.scale.lerp(Vec3::splat(target_scale), 2.0 * dt);
}
