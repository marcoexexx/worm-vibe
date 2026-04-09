/// Player-facing control mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ControlMode {
  #[default]
  Arrow,
  Joystick,
  Mouse,
}

impl ControlMode {
  pub const ALL: [Self; 3] = [Self::Arrow, Self::Joystick, Self::Mouse];

  pub fn label(self) -> &'static str {
    match self {
      Self::Arrow => "ARROW",
      Self::Joystick => "JOYSTICK",
      Self::Mouse => "MOUSE",
    }
  }

  pub fn hint(self) -> &'static str {
    match self {
      Self::Arrow => "WASD / Arrows",
      Self::Joystick => "Touch stick",
      Self::Mouse => "Follow cursor",
    }
  }
}

/// Which side of the screen holds the boost button (touch input).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BoostSide {
  #[default]
  Left,
  Right,
}

impl BoostSide {
  pub const ALL: [Self; 2] = [Self::Left, Self::Right];

  pub fn label(self) -> &'static str {
    match self {
      Self::Left => "LEFT",
      Self::Right => "RIGHT",
    }
  }
}

/// Pure-data game settings — no Bevy dependencies.
#[derive(Debug, Clone)]
pub struct GameSettings {
  pub control_mode: ControlMode,
  pub boost_side: BoostSide,
  pub sound_on: bool,
  pub zoom_level: f32,
}

impl Default for GameSettings {
  fn default() -> Self {
    Self {
      control_mode: ControlMode::default(),
      boost_side: BoostSide::default(),
      sound_on: true,
      zoom_level: 1.0,
    }
  }
}

impl GameSettings {
  pub const ZOOM_MIN: f32 = 0.5;
  pub const ZOOM_MAX: f32 = 2.0;
  pub const ZOOM_STEP: f32 = 0.25;

  pub fn zoom_in(&mut self) {
    self.zoom_level = (self.zoom_level + Self::ZOOM_STEP).min(Self::ZOOM_MAX);
  }

  pub fn zoom_out(&mut self) {
    self.zoom_level = (self.zoom_level - Self::ZOOM_STEP).max(Self::ZOOM_MIN);
  }
}
