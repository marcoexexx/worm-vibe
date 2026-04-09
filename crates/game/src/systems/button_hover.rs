use bevy::prelude::*;
use rendering::theme::GruvboxTheme;

pub(crate) struct ButtonHoverPlugin;

impl Plugin for ButtonHoverPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, handle_button_hover);
  }
}

#[allow(clippy::type_complexity)]
fn handle_button_hover(
  mut buttons: Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>)>,
  mut texts: Query<&mut TextColor>,
  theme: Res<GruvboxTheme>,
) {
  for (interaction, mut bg, children) in &mut buttons {
    let (new_bg, new_text) = match *interaction {
      Interaction::Pressed => (theme.orange, theme.bg_hard),
      Interaction::Hovered => (theme.yellow, theme.bg_hard),
      Interaction::None => (theme.bg_soft, theme.fg),
    };

    *bg = BackgroundColor(new_bg);

    // Update child text color
    for child in children.iter() {
      if let Ok(mut text_color) = texts.get_mut(*child) {
        *text_color = TextColor(new_text);
      }
    }
  }
}
