use bevy::prelude::*;

/// Loaded font handles shared across all UI.
#[derive(Resource)]
pub struct GameFonts {
  pub regular: Handle<Font>,
  pub bold: Handle<Font>,
}

pub(crate) struct FontPlugin;

impl Plugin for FontPlugin {
  fn build(&self, app: &mut App) {
    // Load fonts immediately during plugin build so they're available
    // before any OnEnter systems run (including the default state).
    let asset_server = app.world().resource::<AssetServer>();
    let regular = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
    let bold = asset_server.load("fonts/JetBrainsMono-Bold.ttf");
    app.insert_resource(GameFonts { regular, bold });
  }
}
