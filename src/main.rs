mod scene_spawn;
mod player_control;
mod camera_spawn;

use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy::prelude::*;


pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {

    App::new()

        .insert_resource(ClearColor(Color::hex("83CEF7").unwrap()))

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

        .add_plugins(camera_spawn::SpawnCameraPlugins)
        .add_plugin(scene_spawn::SpawnBasicScenePlugin)
        .add_plugin(player_control::PlayerPlugin)

        .add_plugin(WorldInspectorPlugin)

        .run();
}
