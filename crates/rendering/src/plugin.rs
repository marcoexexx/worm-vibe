use bevy::prelude::*;

use crate::arena_renderer::ArenaRendererPlugin;
use crate::audio::GameAudioPlugin;
use crate::camera::CameraPlugin;
use crate::effects::EffectsPlugin;
use crate::fonts::FontPlugin;
use crate::food_renderer::FoodRendererPlugin;
use crate::hud::HudPlugin;
use crate::menu::MenuPlugin;
use crate::theme::GruvboxTheme;
use crate::worm_renderer::WormRendererPlugin;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<GruvboxTheme>().add_plugins((
      FontPlugin,
      GameAudioPlugin,
      CameraPlugin,
      ArenaRendererPlugin,
      WormRendererPlugin,
      FoodRendererPlugin,
      HudPlugin,
      MenuPlugin,
      EffectsPlugin,
    ));
  }
}
