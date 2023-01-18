use bevy::prelude::*;

use std::f32::consts::PI;
use bevy_rapier3d::prelude::*;

pub struct SpawnBasicScenePlugin;

impl Plugin for SpawnBasicScenePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup,asset_loading)

            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration{
                gravity: Vec3::new(0.0,-10.0,0.0),
                ..default()
            })

            .add_startup_system(spawn_basic_scene)

            .add_plugin(RapierDebugRenderPlugin::default())

        ;

    }
}

#[derive(Resource)]
pub struct GameAssets{
    plant: Handle<Scene>,
    grass_cube: Handle<Scene>,
}

fn spawn_block(
    commands: &mut Commands,
    scene: Handle<Scene>,
    x: f32,
    y: f32,
    z: f32,
){
    commands.spawn((
        SceneBundle{
            scene,
            transform: Transform::from_xyz(x,y,z),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5,0.5,0.5),
    ));
}

fn spawn_basic_scene(
    mut commands: Commands,
    assets: Res<GameAssets>,
){

    for x in 0..10 {
        for z in 0..10 {
            spawn_block(&mut commands,assets.grass_cube.clone(),x as f32,0.0, z as f32);
        }
    }

    for x in 4..6 {
        for z in 4..6 {
            spawn_block(&mut commands,assets.grass_cube.clone(),x as f32,1.0, z as f32);
        }
    }

    for x in 1..3 {
        for z in 1..3 {
            spawn_flora(&mut commands, assets.plant.clone(), x as f32,1.0, z as f32);
        }
    }

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

fn spawn_flora(
    commands: &mut Commands,
    scene: Handle<Scene>,
    x: f32,
    y: f32,
    z: f32
) {
    commands.spawn((
        SceneBundle {
            scene,
            transform: Transform::from_xyz(x, y, z),
            ..default()
        },
        RigidBody::Fixed,
    ))
        .with_children(|parent| {
            parent.spawn((
                TransformBundle {
                    local: Transform::from_xyz(0.0, -0.4, 0.0),
                    ..default()
                },
                Collider::cuboid(0.2, 0.1, 0.2),
            ));
        });
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

