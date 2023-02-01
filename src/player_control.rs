use bevy::input::mouse::MouseMotion;
use crate::*;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system(follow::<camera_spawn::MainCam,PlayerHead>)

            .add_startup_system(spawn_player)

            .add_system(move_player)
            .add_system(turn_player)
            .add_system(jump_player)
            .add_system(grab_mouse_control)
            .add_system(jump_check)
            .add_system(interact_raycast)
            .add_system(highlight_target)

            .add_event::<RayHitEvent>()
            .add_event::<InteractEvent>()

            .register_type::<GroundControl>()
        ;
    }
}

#[derive(Component, Reflect)]
pub struct GroundControl{
    state: bool,
    entities: Vec<Entity>,
}

#[derive(Component)]
pub struct RayHit;

pub struct RayHitEvent(Option<Entity>);

pub struct InteractEvent(Entity);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHead;

#[derive(Component)]
pub struct PlayerRay;

#[derive(Component)]
pub struct PlayerFeet;

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

fn highlight_target(
    mut ev_rh : EventReader<RayHitEvent>,
    query_et : Query<(Entity, &GlobalTransform), Without<RayHit>>,
    mut query_t_rh : Query<&mut Transform, With<RayHit>>,
){
    for ev in ev_rh.iter() {
        let mut trans_rh = query_t_rh.single_mut();

        if let Some(hit_e) = ev.0 {

            for (entity, global_trans) in query_et.iter() {
                if entity == hit_e {
                    let (_, _, trans) = global_trans.to_scale_rotation_translation();
                    trans_rh.translation = trans;
                }
            }
        }
        else{
            trans_rh.translation = Vec3::ZERO;
        }
    }
}

fn interact_raycast(
    rapier_context: Res<RapierContext>,
    query_gt_pr : Query<&GlobalTransform, (With<PlayerRay>, Without<PlayerHead>)>,
    query_gt_ph : Query<&GlobalTransform, (Without<PlayerRay>, With<PlayerHead>)>,
    query_e_p : Query<Entity, With<Player>>,
    mut ev_rh: EventWriter<RayHitEvent>,
    mut ev_i: EventWriter<InteractEvent>,
    mouse: Res<Input<MouseButton>>,
){
    let max_toi = 4.0;

    let globaltrans_pr = query_gt_pr.single();
    let globaltrans_ph = query_gt_ph.single();
    let entity_p = query_e_p.single();

    let (_,_,ray_pos) = globaltrans_ph.to_scale_rotation_translation();
    let (_,_,ray_off) = globaltrans_pr.to_scale_rotation_translation();

    let ray_dir = ray_off - ray_pos;

    let filter = QueryFilter::new()
        .exclude_rigid_body(entity_p)
        .exclude_sensors()
        ;


    if let Some((entity, _)) = rapier_context.cast_ray(
        ray_pos,
        ray_dir,
        max_toi,
        true,
        filter,
    ){
        ev_rh.send(RayHitEvent(Some(entity)));
        if mouse.just_pressed(MouseButton::Left) {
            ev_i.send(InteractEvent(entity));
        }
    }
    else{
        ev_rh.send(RayHitEvent(None));
    }
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
            Name::new("Player"),
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
            )
                .with_children(|parent|{
                    parent.spawn(
                        (TransformBundle{
                            local: Transform::from_xyz(0.0,0.0,-1.0),
                            ..default()
                        },
                         PlayerRay,
                        )
                    );
                });
            parent.spawn(
                (
                    TransformBundle{
                        local: Transform::from_xyz(0.0,-0.5,0.0),
                        ..default()
                    },
                    PlayerFeet,
                    GroundControl{state:false,entities:Vec::with_capacity(4)},
                    Collider::cuboid(0.1,0.1,0.1),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                )
            );

        })
    ;

    commands.spawn((

        TransformBundle{
            ..default()
        },
        Collider::ball(0.7),
        Sensor,
        RayHit,
        Name::new("Ray Hit"),
        ));

}

fn jump_check(
    mut query_f: Query<(Entity,&mut GroundControl), With<PlayerFeet>>,
    mut contact_events: EventReader<CollisionEvent>,
){
    let (f_entity, mut f_g_c) = query_f.single_mut();

    f_g_c.state = !f_g_c.entities.is_empty();

    for contact_event in contact_events.iter(){

        if let CollisionEvent::Started(e1,e2,_) = contact_event {
            if e2 == &f_entity {
                f_g_c.entities.push(*e1);
            }

            if e1 == &f_entity{
                f_g_c.entities.push(*e2);
            }
        }

        if let CollisionEvent::Stopped(e1,e2,_) = contact_event {
            if e2 == &f_entity  {
                f_g_c.entities.retain(|&e| e != *e1)
            }
            if e1 == &f_entity  {
                f_g_c.entities.retain(|&e| e != *e2)
            }
        }
    }
}

fn jump_player(
    mut query_p_impulse: Query<&mut ExternalImpulse, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    query_gc: Query<&GroundControl>,
){
    let ground_control = query_gc.single();
    if !ground_control.state {return;}

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
    let mut m_vec = Vec3::new(0.0,0.0,0.0);

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

    if m_vec.length() > 0.0{
        m_vec = m_vec.normalize();
    }

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