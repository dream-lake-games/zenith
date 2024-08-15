use bevy::reflect::GetTypeRegistration;
use dphysics::ShowPhysicsBounds;

use crate::prelude::*;

use super::CollisionsSet;

pub trait Patrollable:
    Component + std::fmt::Debug + Clone + Reflect + FromReflect + TypePath + GetTypeRegistration
{
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct DefaultPatrollable;

impl Patrollable for DefaultPatrollable {}

/// Watches for a TRIGGER_TX (NOTENOTENOTE) in VISION with C
/// Why? We already go through the logic of duplicating trigger tx for room
#[derive(Component, Debug, Clone, Reflect)]
pub struct PatrolWatch<C: Patrollable, M: Patrollable = DefaultPatrollable> {
    vision: Bounds,
    _ignore: Option<C>,
    _more_ignore: Option<M>,
}
impl<C: Patrollable, M: Patrollable> PatrolWatch<C, M> {
    pub fn new(vision: Bounds) -> Self {
        Self {
            vision,
            _ignore: None,
            _more_ignore: None,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct PatrolActive<M: Patrollable = DefaultPatrollable> {
    pub target_eid: Entity,
    pub time_seen: f32,
    _ignore: Option<M>,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct PatrolInactive<M: Patrollable = DefaultPatrollable> {
    _ignore: Option<M>,
}

fn find_all_in_vision<C: Patrollable, M: Patrollable>(
    target_q: &Query<(Entity, &GlobalTransform, &TriggerTx), With<C>>,
    watch: &PatrolWatch<C, M>,
    my_pos: Vec2,
    my_angle: f32,
) -> Vec<Entity> {
    let mut result = vec![];
    for (eid, gtran, trigger_tx) in target_q {
        let (other_pos, other_ang) = gtran.pos_n_angle();
        if watch
            .vision
            .overlap_out(
                (my_pos, my_angle),
                (&trigger_tx.bounds, other_pos, other_ang),
            )
            .is_some()
        {
            result.push(eid);
        }
    }
    result
}

fn find_all_seen<C: Patrollable, M: Patrollable>(
    target_q: &Query<(Entity, &GlobalTransform, &TriggerTx), With<C>>,
    _static_q: &Query<&StaticTx>,
    watch: &PatrolWatch<C, M>,
    my_pos: Vec2,
    my_angle: f32,
) -> Vec<Entity> {
    let in_vision = find_all_in_vision(target_q, watch, my_pos, my_angle);
    // TODO: Come up with a "ray intersects line segment" fn that returns Option<point of intersection>
    // then you can find out if it's actually seeable by looking through edges
    in_vision
}

fn draw_patrols<C: Patrollable, M: Patrollable>(
    patrol_q: Query<(&PatrolWatch<C, M>, &GlobalTransform)>,
    mut gz: Gizmos,
    meta_state: Res<State<MetaState>>,
) {
    let mut offsets = vec![Vec2::default()];
    if let Some(room_state) = meta_state.get_room_state() {
        offsets.extend(room_state.mirage_offsets());
    }
    for (watch, gtran) in &patrol_q {
        let (pos, rot) = gtran.pos_n_angle();
        // Draw it at all the places it's looking, even tho in reality it's only looking in the canonical space
        // and it's relying on it's target having duped trigger_tx s
        for offset in offsets.clone() {
            watch
                .vision
                .draw(pos + offset, rot, &mut gz, tailwind::YELLOW_400.into());
        }
    }
}

fn update_patrols<C: Patrollable, M: Patrollable>(
    target_q: Query<(Entity, &GlobalTransform, &TriggerTx), With<C>>,
    static_q: Query<&StaticTx>,
    mut patrol_watch: Query<(
        Entity,
        &PatrolWatch<C, M>,
        Option<&mut PatrolActive>,
        &GlobalTransform,
    )>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
) {
    for (eid, watch, mut active, gtran) in &mut patrol_watch {
        let (my_pos, my_angle) = gtran.pos_n_angle();
        let mut seen_targets = find_all_seen(&target_q, &static_q, watch, my_pos, my_angle);
        if seen_targets.len() == 0 {
            commands.entity(eid).remove::<PatrolActive<M>>();
            commands
                .entity(eid)
                .insert(PatrolInactive::<M> { _ignore: None });
        } else {
            commands.entity(eid).remove::<PatrolInactive<M>>();
            match active.as_mut() {
                Some(old_active) => {
                    if seen_targets.contains(&old_active.target_eid) {
                        old_active.time_seen += bullet_time.delta_seconds();
                    } else {
                        old_active.target_eid = seen_targets.pop().unwrap();
                        old_active.time_seen = 0.0;
                    }
                }
                None => {
                    commands.entity(eid).insert(PatrolActive::<M> {
                        target_eid: seen_targets.pop().unwrap(),
                        time_seen: 0.0,
                        _ignore: None,
                    });
                }
            };
        }
    }
}

pub fn register_patrol<C: Patrollable, M: Patrollable>(app: &mut App) {
    app.register_type::<PatrolWatch<C, M>>();
    app.register_type::<PatrolActive<M>>();
    app.register_type::<PatrolInactive<M>>();

    app.add_systems(
        PostUpdate,
        draw_patrols::<C, M>.run_if(in_state(ShowPhysicsBounds)),
    );
    app.add_systems(
        Update,
        update_patrols::<C, M>
            .in_set(PhysicsSet)
            .after(CollisionsSet),
    );
}
