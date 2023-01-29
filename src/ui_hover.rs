use bevy::prelude::*;

use crate::*;

pub struct UiHoverPlugin;

impl Plugin for UiHoverPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_ui_cam)
            .add_startup_system(spawn_ui_hover)
            .add_system(ui_follow::<UiHover,player_control::RayHit>)
        ;
    }
}

#[derive(Component)]
pub struct UiCam;

#[derive(Component)]
pub struct UiHover;

fn spawn_ui_cam(
    mut commands: Commands,
){
    commands.spawn((
        Camera2dBundle {
            camera: Camera{
                ..default()
            },
            ..default()
        }
        ,UiCam));
}

fn ui_follow<F:Component,T:Component>(
    query_t_trans: Query<&GlobalTransform,(With<T>,Without<F>)>,
    mut query_f_style: Query<&mut Style, (With<F>,Without<T>)>,
    query_uic_c: Query<&Camera, With<camera_spawn::MainCam>>,
    query_c_gt: Query<&GlobalTransform, With<camera_spawn::MainCam>>,

){
    let t_trans = query_t_trans.single();
    let mut f_style = query_f_style.single_mut();
    let cam = query_uic_c.single();
    let cam_trans = query_c_gt.single();

    let (_,_,trans) = t_trans.to_scale_rotation_translation();

    if let Some(view_pos) = cam.world_to_viewport(cam_trans, trans){
        f_style.position = UiRect{
            bottom: Val::Px(view_pos.y - HEIGHT/2.0),
            left: Val::Px(view_pos.x - WIDTH/2.0),
            ..default()
        };
    }
}

fn spawn_ui_hover(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands.spawn((
        UiHover,
        Name::new("Test"),
        NodeBundle{
            style: Style{
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(60.0),Val::Px(40.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        }
        )).with_children(|parent|{
        parent.spawn((
            NodeBundle{
                background_color: BackgroundColor(Color::rgb(1.0,1.0,1.0)),
                style: Style{
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(60.0),Val::Px(40.0)),
                    position: UiRect{
                        left: Val::Px(200.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }
        )).with_children(|parent|{
            parent.spawn((
                TextBundle{
                    text: Text::from_section(
                        "Test",
                        TextStyle{
                            font_size: 20.0,
                            font: asset_server.load("fonts/Lexend-VariableFont_wght.ttf"),
                            color: Color::rgb(0.0,0.0,0.0),
                            ..default()
                        }
                    ),
                    ..default()
                }
            ));
        });
    });
}