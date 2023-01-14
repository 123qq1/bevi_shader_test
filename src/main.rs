use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {

    App::new()

        .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))

        .add_startup_system_to_stage(StartupStage::PreStartup,asset_loading)

        .add_startup_system(spawn_basic_scene)
        //.add_startup_system(spawn_camera)

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height:HEIGHT,
                title: "Shader Test".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))

        .add_plugin(WorldInspectorPlugin::new())

        .run();
}

#[derive(Resource)]
pub struct GameAssets{
    main_scene: Handle<Scene>,
}

pub struct PixelMaterial{

}

fn spawn_camera(
    mut commands: Commands
){
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(-2.0,2.5,5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    assets: Res<GameAssets>,
){

    commands.spawn(SceneBundle{
        scene: assets.main_scene.clone(),
        ..default()
    });
}

fn asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>,
){
    commands.insert_resource(GameAssets{
        main_scene: assets.load("BaseScene.glb#Scene0"),
    });
}
