use bevy::prelude::*;

/// Dark Gruvbox color palette.
#[derive(Resource, Clone)]
pub struct GruvboxTheme {
  pub bg_hard: Color,
  pub bg: Color,
  pub bg_soft: Color,
  pub fg: Color,
  pub gray: Color,
  pub red: Color,
  pub green: Color,
  pub yellow: Color,
  pub blue: Color,
  pub purple: Color,
  pub aqua: Color,
  pub orange: Color,
}

impl Default for GruvboxTheme {
  fn default() -> Self {
    Self {
      bg_hard: Color::srgb_u8(0x1d, 0x20, 0x21),
      bg: Color::srgb_u8(0x28, 0x28, 0x28),
      bg_soft: Color::srgb_u8(0x32, 0x30, 0x2f),
      fg: Color::srgb_u8(0xeb, 0xdb, 0xb2),
      gray: Color::srgb_u8(0x92, 0x83, 0x74),
      red: Color::srgb_u8(0xcc, 0x24, 0x1d),
      green: Color::srgb_u8(0x98, 0x97, 0x1a),
      yellow: Color::srgb_u8(0xd7, 0x99, 0x21),
      blue: Color::srgb_u8(0x45, 0x85, 0x88),
      purple: Color::srgb_u8(0xb1, 0x62, 0x86),
      aqua: Color::srgb_u8(0x68, 0x9d, 0x6a),
      orange: Color::srgb_u8(0xd6, 0x5d, 0x0e),
    }
  }
}

impl GruvboxTheme {
  /// Pick a worm color by index (cycles through palette).
  pub fn worm_color(&self, index: usize) -> Color {
    let palette = [self.red, self.yellow, self.blue, self.purple, self.orange, self.aqua];
    palette[index % palette.len()]
  }

  /// Player worm always gets green/aqua.
  pub fn player_color(&self) -> Color {
    self.green
  }
}
