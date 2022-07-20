use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct FabienPlugin;

#[derive(Clone)]
pub struct FabienSheet(Handle<TextureAtlas>);

impl Plugin  for FabienPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                load_fabien
            );

    }
}

pub fn spawn_fabien_sprite(
    commands: &mut Commands,
    fabien: &FabienSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity 
{
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: fabien.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn load_fabien(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
)
{
    let image = assets.load("big_sprite_sheet.png");
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(25.0),
        4,
        7,
    );

    let atlas_handle
        = texture_atlases.add(atlas);
    
    commands.insert_resource(FabienSheet(atlas_handle));
}