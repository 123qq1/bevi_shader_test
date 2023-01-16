use bevy::input::mouse::MouseMotion;
use bevy::math::vec3;
use crate::*;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system(follow::<scene_spawn::MainCam,PlayerHead>)
            .add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(turn_player)
            .add_system(jump_player)
            .add_system(grab_mouse_control)
        ;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHead;

pub fn follow<F:Component,T:Component>(
    query_t_trans: Query<&GlobalTransform,(With<T>,Without<F>)>,
    mut query_f_trans: Query<&mut Transform, (With<F>,Without<T>)>
){
    let t_trans = query_t_trans.single();
    let mut f_trans = query_f_trans.single_mut();

    let (_,rot,trans) = t_trans.to_scale_rotation_translation();

    f_trans.translation = trans;
    f_trans.rotation = rot;
}

fn spawn_player(
    mut commands: Commands,
){
    commands.spawn(
        (
            TransformBundle{
                local: Transform::from_xyz(5.0,2.0,5.0),
                ..default()
            },
            Player,
            RigidBody::Dynamic,
            Collider::cuboid(0.2,0.5,0.2),
            ColliderMassProperties::Density(1.0),
            LockedAxes::ROTATION_LOCKED,
            ExternalImpulse {
                impulse: Vec3::ZERO,
                torque_impulse: Vec3::ZERO,
            },
            )

        )
        .with_children(|parent|{
            parent.spawn(
                (
                    TransformBundle{
                       local: Transform::from_xyz(0.0,0.45,0.0),
                        ..default()
                    },
                    PlayerHead,
                )
            );
        });
}

fn jump_player(
    mut query_p_impulse: Query<&mut ExternalImpulse, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
){
    let jump_pow = 0.8;
    let mut p_impulse = query_p_impulse.single_mut();

    if keyboard.just_pressed(KeyCode::Space){
            p_impulse.impulse = Vec3::new(0.0, jump_pow, 0.0);
    }
}

fn move_player(
    mut query_p_trans: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    let speed = 2.0;
    let mut p_trans = query_p_trans.single_mut();

    let speed_multi = speed * time.delta_seconds();
    let mut m_vec = vec3(0.0,0.0,0.0);

    if keyboard.pressed(KeyCode::A){
        m_vec += p_trans.left();
    }

    if keyboard.pressed(KeyCode::D){
        m_vec += p_trans.right();
    }

    if keyboard.pressed(KeyCode::W){
        m_vec += p_trans.forward();
    }

    if keyboard.pressed(KeyCode::S){
        m_vec += p_trans.back();
    }

    //m_vec = m_vec.normalize();

    m_vec = m_vec * speed_multi;

    p_trans.translation += m_vec;
}

fn turn_player(
    mut query_p_trans: Query<&mut Transform, (With<Player>, Without<PlayerHead>)>,
    mut query_ph_trans: Query<&mut Transform, (With<PlayerHead>, Without<Player>)>,

    mut mous_ev: EventReader<MouseMotion>,
){
    let turn_speed = 0.001;
    let mut p_trans = query_p_trans.single_mut();
    let mut ph_trans = query_ph_trans.single_mut();
    for ev in mous_ev.iter(){
        p_trans.rotate_y(-ev.delta.x * turn_speed);
        ph_trans.rotate_local_x(-ev.delta.y * turn_speed);
    }
}

fn grab_mouse_control(
    mut windows: ResMut<Windows>,
    key: Res<Input<KeyCode>>,
){
    let window = windows.get_primary_mut().unwrap();

    if key.just_pressed(KeyCode::Escape){

        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }

    if key.just_pressed(KeyCode::Tab){

        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }
}