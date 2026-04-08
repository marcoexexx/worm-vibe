use app::GameWorld;
use bevy::prelude::*;
use domain::FoodKind;

#[derive(Component)]
struct FoodSprite;

/// Pre-loaded food textures for all food kinds.
#[derive(Resource)]
struct FoodTextures {
  donut: Handle<Image>,
  cookie: Handle<Image>,
  cherry: Handle<Image>,
  banana: Handle<Image>,
  apple: Handle<Image>,
  grape: Handle<Image>,
  watermelon: Handle<Image>,
  strawberry: Handle<Image>,
}

/// Tracks spawned food entities.
#[derive(Resource, Default)]
struct FoodSpriteRegistry {
  entities: Vec<Entity>,
}

pub(crate) struct FoodRendererPlugin;

impl Plugin for FoodRendererPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<FoodSpriteRegistry>()
      .add_systems(PreStartup, load_food_textures)
      .add_systems(Update, sync_food_sprites);
  }
}

fn load_food_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.insert_resource(FoodTextures {
    donut: asset_server.load("textures/food_donut.png"),
    cookie: asset_server.load("textures/food_cookie.png"),
    cherry: asset_server.load("textures/food_cherry.png"),
    banana: asset_server.load("textures/food_banana.png"),
    apple: asset_server.load("textures/food_apple.png"),
    grape: asset_server.load("textures/food_grape.png"),
    watermelon: asset_server.load("textures/food_watermelon.png"),
    strawberry: asset_server.load("textures/food_strawberry.png"),
  });
}

fn sync_food_sprites(
  mut commands: Commands,
  world: Option<Res<GameWorld>>,
  mut registry: ResMut<FoodSpriteRegistry>,
  mut transforms: Query<(&mut Transform, &mut Sprite), With<FoodSprite>>,
  textures: Option<Res<FoodTextures>>,
) {
  let Some(world) = world else { return };
  let Some(textures) = textures else { return };

  let foods = world.foods();
  let target_count = foods.len();

  // Despawn excess
  if registry.entities.len() > target_count {
    for entity in registry.entities.drain(target_count..) {
      commands.entity(entity).despawn();
    }
  }

  // Update existing
  for (i, food) in foods.iter().enumerate().take(registry.entities.len()) {
    let entity = registry.entities[i];
    if let Ok((mut transform, mut sprite)) = transforms.get_mut(entity) {
      transform.translation = food.position().extend(1.0);
      sprite.image = food_texture(food.kind(), &textures);
      sprite.custom_size = Some(Vec2::splat(food.radius() * 2.5));
    }
  }

  // Spawn new
  for food in foods.iter().skip(registry.entities.len()) {
    let size = food.radius() * 2.5;
    let entity = commands
      .spawn((
        Sprite {
          image: food_texture(food.kind(), &textures),
          custom_size: Some(Vec2::splat(size)),
          ..default()
        },
        Transform::from_translation(food.position().extend(1.0)),
        FoodSprite,
      ))
      .id();
    registry.entities.push(entity);
  }
}

fn food_texture(kind: FoodKind, textures: &FoodTextures) -> Handle<Image> {
  match kind {
    FoodKind::Donut => textures.donut.clone(),
    FoodKind::Cookie => textures.cookie.clone(),
    FoodKind::Cherry => textures.cherry.clone(),
    FoodKind::Banana => textures.banana.clone(),
    FoodKind::Apple => textures.apple.clone(),
    FoodKind::Grape => textures.grape.clone(),
    FoodKind::Watermelon => textures.watermelon.clone(),
    FoodKind::Strawberry => textures.strawberry.clone(),
  }
}
