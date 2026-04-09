use bevy::prelude::*;
use input::VirtualJoystick;

const OUTER_DIAMETER: f32 = 160.0;
const INNER_DIAMETER: f32 = 60.0;
const OUTER_ALPHA: f32 = 0.25;
const INNER_ALPHA: f32 = 0.4;
#[derive(Component)]
struct JoystickBase;

#[derive(Component)]
struct JoystickThumb;

pub(crate) struct JoystickOverlayPlugin;

impl Plugin for JoystickOverlayPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_overlay)
      .add_systems(Update, update_overlay);
  }
}

fn spawn_overlay(mut commands: Commands) {
  let outer_color = Color::srgba(1.0, 1.0, 1.0, OUTER_ALPHA);
  let inner_color = Color::srgba(1.0, 1.0, 1.0, INNER_ALPHA);
  let border_color = Color::srgba(1.0, 1.0, 1.0, OUTER_ALPHA + 0.1);

  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      width: Val::Px(OUTER_DIAMETER),
      height: Val::Px(OUTER_DIAMETER),
      left: Val::Px(0.0),
      top: Val::Px(0.0),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BackgroundColor(outer_color),
    BorderColor(border_color),
    BorderRadius::all(Val::Percent(50.0)),
    Visibility::Hidden,
    JoystickBase,
  ));

  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      width: Val::Px(INNER_DIAMETER),
      height: Val::Px(INNER_DIAMETER),
      left: Val::Px(0.0),
      top: Val::Px(0.0),
      ..default()
    },
    BackgroundColor(inner_color),
    BorderRadius::all(Val::Percent(50.0)),
    Visibility::Hidden,
    JoystickThumb,
  ));
}

#[allow(clippy::type_complexity)]
fn update_overlay(
  joystick: Res<VirtualJoystick>,
  mut base_q: Query<(&mut Node, &mut Visibility), (With<JoystickBase>, Without<JoystickThumb>)>,
  mut thumb_q: Query<(&mut Node, &mut Visibility), (With<JoystickThumb>, Without<JoystickBase>)>,
) {
  let Ok((mut base_node, mut base_vis)) = base_q.get_single_mut() else {
    return;
  };
  let Ok((mut thumb_node, mut thumb_vis)) = thumb_q.get_single_mut() else {
    return;
  };

  if !joystick.is_active() {
    *base_vis = Visibility::Hidden;
    *thumb_vis = Visibility::Hidden;
    return;
  }

  let Some(center) = joystick.center() else {
    return;
  };
  let Some(current) = joystick.current() else {
    return;
  };

  *base_vis = Visibility::Inherited;
  *thumb_vis = Visibility::Inherited;

  let half_outer = OUTER_DIAMETER / 2.0;
  base_node.left = Val::Px(center.x - half_outer);
  base_node.top = Val::Px(center.y - half_outer);

  let delta = current - center;
  let radius = joystick.radius();
  let clamped = if delta.length() > radius {
    delta.normalize() * radius
  } else {
    delta
  };

  let thumb_pos = center + clamped;
  let half_inner = INNER_DIAMETER / 2.0;
  thumb_node.left = Val::Px(thumb_pos.x - half_inner);
  thumb_node.top = Val::Px(thumb_pos.y - half_inner);
}
