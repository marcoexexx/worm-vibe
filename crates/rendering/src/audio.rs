use bevy::prelude::*;
use domain::GameSettings;

/// Pre-loaded sound effect handles.
#[derive(Resource)]
pub struct GameSounds {
  pub eat: Handle<AudioSource>,
  pub die: Handle<AudioSource>,
  pub boost_start: Handle<AudioSource>,
  pub menu_click: Handle<AudioSource>,
}

pub(crate) struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PreStartup, load_sounds);
  }
}

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.insert_resource(GameSounds {
    eat: asset_server.load("sounds/eat.ogg"),
    die: asset_server.load("sounds/die.ogg"),
    boost_start: asset_server.load("sounds/boost.ogg"),
    menu_click: asset_server.load("sounds/click.ogg"),
  });
}

/// Play a one-shot sound effect, respecting the global sound setting.
pub fn play_sfx(commands: &mut Commands, sound: &Handle<AudioSource>, settings: &GameSettings) {
  if !settings.sound_on {
    return;
  }
  commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
}
