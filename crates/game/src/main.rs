mod state;
mod systems;

use bevy::prelude::*;
use rendering::theme::GruvboxTheme;

fn main() {
  let asset_path = if cfg!(target_arch = "wasm32") {
    "assets".to_string()
  } else {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets").to_string()
  };

  let window = if cfg!(target_arch = "wasm32") {
    Window {
      title: "WORMZONE".into(),
      fit_canvas_to_parent: true,
      prevent_default_event_handling: true,
      ..default()
    }
  } else {
    Window {
      title: "WORMZONE // dev build".into(),
      resolution: (1280.0, 720.0).into(),
      ..default()
    }
  };

  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(window),
          ..default()
        })
        .set(AssetPlugin {
          file_path: asset_path,
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
