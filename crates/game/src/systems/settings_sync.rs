//! Syncs `SettingsRes` (rendering) → `ActiveControlMode` (input).

use bevy::prelude::*;
use input::ActiveControlMode;
use rendering::menu::settings_menu::SettingsRes;

pub(crate) struct SettingsSyncPlugin;

impl Plugin for SettingsSyncPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreUpdate, sync_control_mode.run_if(resource_changed::<SettingsRes>));
  }
}

fn sync_control_mode(settings: Res<SettingsRes>, mut mode: ResMut<ActiveControlMode>) {
  mode.0 = settings.control_mode;
}
