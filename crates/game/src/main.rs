mod state;
mod systems;

use bevy::prelude::*;
use rendering::theme::GruvboxTheme;

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "WORMZONE // dev build".into(),
            resolution: (1280.0, 720.0).into(),
            ..default()
          }),
          ..default()
        })
        .set(AssetPlugin {
          file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets").to_string(),
          ..default()
        }),
    )
    .insert_resource(ClearColor(GruvboxTheme::default().bg))
    .init_state::<state::AppState>()
    .add_plugins((
      input::InputPlugin,
      rendering::RenderingPlugin,
      systems::GameSystemsPlugin,
    ))
    .run();
}
