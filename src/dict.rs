
use bevy::prelude::*;

pub struct LoadAssets;

impl Plugin for LoadAssets{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup,asset_loading)
        ;

    }
}

#[derive(Resource)]
pub struct GameAssets{
    pub plant: Handle<Scene>,
    pub grass_cube: Handle<Scene>,
}



fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>,
){
    commands.insert_resource(GameAssets{
        plant: assets.load("Plant.glb#Scene0"),
        grass_cube: assets.load("Grass.glb#Scene0"),
    });
}