use crate::prelude::*;

use super::{CollisionsSet, PhysicsInitialized};

/// When moving `DynoTran`s that have a vel with mag greater than this number, the movement will
/// occur in steps of this length to resolve collisions for fast-moving objects.
const MAX_TRAN_STEP_LENGTH: f32 = 2.0;

/// Resets all records (collisions + triggers). Happens during PreUpdate
fn reset_collision_records(
    mut statics_provider_q: Query<&mut StaticTx>,
    mut statics_receiver_q: Query<&mut StaticRx>,
    mut triggers_provider_q: Query<&mut TriggerTx>,
    mut triggers_receiver_q: Query<&mut TriggerRx>,
    collision_root: Res<CollisionRoot>,
    mut commands: Commands,
) {
    for mut provider in statics_provider_q.iter_mut() {
        provider.collisions = VecDeque::new();
    }
    for mut receiver in statics_receiver_q.iter_mut() {
        receiver.collisions = VecDeque::new();
    }
    for mut provider in triggers_provider_q.iter_mut() {
        provider.collisions = VecDeque::new();
    }
    for mut receiver in triggers_receiver_q.iter_mut() {
        receiver.collisions = VecDeque::new();
    }
    commands.entity(collision_root.eid()).despawn_descendants();
}

/// Enforces current limitations in the physics system by panicking if I ever fuck up.
fn enforce_invariants(
    provider_and_receiver: Query<Entity, (With<StaticTx>, With<StaticRx>)>,
    trigger_on_static: Query<Entity, (Or<(With<TriggerTx>, With<TriggerRx>)>, With<StaticTx>)>,
    no_gtran: Query<
        Entity,
        (
            Or<(
                With<StaticTx>,
                With<StaticRx>,
                With<TriggerTx>,
                With<TriggerRx>,
            )>,
            Without<GlobalTransform>,
        ),
    >,
    no_dyno_tran_on_static_receiver: Query<Entity, (With<StaticRx>, Without<DynoTran>)>,
    dyno_rot_on_static_receiver: Query<Entity, (With<StaticRx>, With<DynoRot>)>,
) {
    if !provider_and_receiver.is_empty() {
        panic!("An entity cannot be both a static provider and a static receiver");
    }
    if !trigger_on_static.is_empty() {
        panic!("Trigger receivers on static providers are not yet supported");
    }
    if !no_gtran.is_empty() {
        panic!("No global transform on a static/trigger");
    }
    if !no_dyno_tran_on_static_receiver.is_empty() {
        panic!("No dynotran on static receiver (how is it supposed to move?)");
    }
    if !dyno_rot_on_static_receiver.is_empty() {
        panic!("Cannot yet put a dynoRot on a staticreceiver, sorry");
    }
}

fn initialize_physics(
    mut commands: Commands,
    relevant_eids: Query<
        Entity,
        (
            Or<(
                With<DynoTran>,
                With<DynoRot>,
                With<StaticRx>,
                With<StaticTx>,
                With<TriggerTx>,
                With<TriggerRx>,
            )>,
            Without<PhysicsInitialized>,
        ),
    >,
) {
    for eid in &relevant_eids {
        commands.entity(eid).insert(PhysicsInitialized);
    }
}

/// Moves all dynos (both rot and tran) that are not statics, do not collide with statics, and have no triggers
fn move_uninteresting_dynos(
    bullet_time: Res<BulletTime>,
    mut rot_only_dynos: Query<
        (&DynoRot, &mut Transform),
        (
            Without<DynoTran>,
            Without<StaticTx>,
            Without<StaticRx>,
            Without<TriggerTx>,
            Without<TriggerRx>,
            With<PhysicsInitialized>,
        ),
    >,
    mut both_dynos: Query<
        (&DynoRot, &DynoTran, &mut Transform),
        (
            Without<StaticTx>,
            Without<StaticRx>,
            Without<TriggerTx>,
            Without<TriggerRx>,
            With<PhysicsInitialized>,
        ),
    >,
    mut tran_only_dynos: Query<
        (&DynoTran, &mut Transform),
        (
            Without<DynoRot>,
            Without<StaticTx>,
            Without<StaticRx>,
            Without<TriggerTx>,
            Without<TriggerRx>,
            With<PhysicsInitialized>,
        ),
    >,
) {
    let apply_rotation = |dyno_rot: &DynoRot, tran: &mut Mut<Transform>| {
        tran.rotate_z(dyno_rot.rot * bullet_time.delta_seconds());
    };
    let apply_translation = |dyno_tran: &DynoTran, tran: &mut Mut<Transform>| {
        tran.translation += (dyno_tran.vel * bullet_time.delta_seconds()).extend(0.0);
    };
    for (dyno_rot, mut tran) in &mut rot_only_dynos {
        apply_rotation(dyno_rot, &mut tran);
    }
    for (dyno_rot, dyno_tran, mut tran) in &mut both_dynos {
        apply_rotation(dyno_rot, &mut tran);
        apply_translation(dyno_tran, &mut tran);
    }
    for (dyno_tran, mut tran) in &mut tran_only_dynos {
        apply_translation(dyno_tran, &mut tran);
    }
}

/// Moves all dynos (both rot and tran) that are static providers.
/// NOTE: Trigger support does not yet exist on static providers, i.e. these entities cannot have triggers.
fn move_static_provider_dynos(
    bullet_time: Res<BulletTime>,
    mut rot_only_dynos: Query<
        (&DynoRot, &mut Transform),
        (Without<DynoTran>, With<StaticTx>, With<PhysicsInitialized>),
    >,
    mut both_dynos: Query<
        (&DynoRot, &DynoTran, &mut Transform),
        (With<StaticTx>, With<PhysicsInitialized>),
    >,
    mut tran_only_dynos: Query<
        (&DynoTran, &mut Transform),
        (
            Without<DynoRot>,
            With<StaticTx>,
            Without<StaticRx>,
            With<PhysicsInitialized>,
        ),
    >,
) {
    let apply_rotation = |dyno_rot: &DynoRot, tran: &mut Mut<Transform>| {
        tran.rotate_z(dyno_rot.rot * bullet_time.delta_seconds());
    };
    let apply_translation = |dyno_tran: &DynoTran, tran: &mut Mut<Transform>| {
        tran.translation += (dyno_tran.vel * bullet_time.delta_seconds()).extend(0.0);
    };
    for (dyno_rot, mut tran) in &mut rot_only_dynos {
        apply_rotation(dyno_rot, &mut tran);
    }
    for (dyno_rot, dyno_tran, mut tran) in &mut both_dynos {
        apply_rotation(dyno_rot, &mut tran);
        apply_translation(dyno_tran, &mut tran);
    }
    for (dyno_tran, mut tran) in &mut tran_only_dynos {
        apply_translation(dyno_tran, &mut tran);
    }
}

/// A helper function to resolve static collisions for a single entity. This will do the work of pushing the
/// entity given by eid outside of other entities it's colliding with
fn resolve_static_collisions(
    rx_eid: Entity,
    rx: &mut StaticRx,
    dyno_tran: &mut DynoTran,
    tran: &mut Transform,
    gtran_offset: Vec2,
    providers: &mut Query<(Entity, &mut StaticTx, &GlobalTransform)>,
    commands: &mut Commands,
    collision_root: &CollisionRoot,
) {
    for (tx_eid, mut tx, tx_gtran) in providers {
        // Correct the global/local translation and see if there is a collision
        let my_tran_n_angle = tran.tran_n_angle();
        let my_tran_n_angle = (my_tran_n_angle.0 + gtran_offset, my_tran_n_angle.1);
        let rhs_tran_n_angle = tx_gtran.tran_n_angle();
        let Some((mvmt, cp)) = rx.bounds.bounce_off(
            my_tran_n_angle,
            (&tx.bounds, rhs_tran_n_angle.0, rhs_tran_n_angle.1),
        ) else {
            // These things don't overlap, nothing to do
            continue;
        };

        // Create a collision record
        let old_perp = dyno_tran.vel.dot(mvmt.normalize_or_zero()) * mvmt.normalize_or_zero();
        let old_par = dyno_tran.vel - old_perp;
        let collision_record = StaticCollisionRecord {
            pos: cp,
            rx_perp: old_perp,
            rx_par: old_par,
            tx_eid,
            tx_kind: tx.kind,
            rx_eid,
            rx_kind: rx.kind,
        };
        let collision_eid = commands
            .spawn(StaticCollisionBundle::new(collision_record))
            .set_parent(collision_root.eid())
            .id();
        rx.collisions.push_back(collision_eid);
        tx.collisions.push_back(collision_eid);

        // Then actually move the objects out of each other and handle physics updates
        tran.translation += mvmt.extend(0.0);
        let bounce_with_friction = |vel: Vec2, springiness: f32, friction: f32| -> Vec2 {
            // TODO: All these normalize_or_zero's are probably a bit slow, fix later
            let old_perp = vel.dot(mvmt.normalize_or_zero()) * mvmt.normalize_or_zero();
            let old_par = vel - old_perp;
            let mut new_perp = old_perp * springiness;
            if new_perp.dot(mvmt) < 0.0 {
                new_perp *= -1.0;
            }
            let friction_mult =
                1.0 + vel.normalize_or_zero().dot(mvmt.normalize_or_zero()).abs() * 10.0;
            let new_par = old_par * (1.0 - (friction * friction_mult).min(1.0));
            new_perp + new_par
        };
        match (tx.kind, rx.kind) {
            (_, StaticRxKind::Stop) => {
                dyno_tran.vel = Vec2::ZERO;
            }
            (_, StaticRxKind::GoAround { mult }) => {
                // Try to move perpendicularly around this thing
                // TODO: Come up with a better system so we don't have to do this
                dyno_tran.vel += Vec2::new(mvmt.y, -mvmt.x) * mult as f32;
            }
            (StaticTxKind::Normal, StaticRxKind::Normal) => {
                dyno_tran.vel = bounce_with_friction(dyno_tran.vel, 0.2, 0.03);
            }
            (StaticTxKind::Sticky, StaticRxKind::Normal) => {
                dyno_tran.vel = Vec2::ZERO;
                let stuck_marker = Stuck {
                    parent: tx_eid,
                    my_initial_angle: my_tran_n_angle.1,
                    parent_initial_angle: rhs_tran_n_angle.1,
                    initial_offset: tran.translation.truncate() + gtran_offset - rhs_tran_n_angle.0,
                };
                commands.entity(rx_eid).insert(stuck_marker);
            }
        }
    }
}

/// Resolves trigger collisions. Note that the data is broken up into multiple queries to allow for
/// proper handling in the parent systems.
///
/// I actually believe this has a slight bug. It always uses global transform, which is static all frame.
/// I.e. if bullet a moves and then bullet b goes it will still be checking against bullet a old pos.
/// Ehh probably fine
fn resolve_trigger_collisions(
    eid: Entity,
    rx: &mut TriggerRx,
    gtran: &Transform,
    shared_data: &Query<(Entity, &GlobalTransform)>,
    trigger_txs: &mut Query<(Entity, &mut TriggerTx)>,
    commands: &mut Commands,
    collision_root: &CollisionRoot,
    dup_set: &mut HashSet<(Entity, Entity)>,
) {
    for (other_eid, mut other_tx) in trigger_txs {
        if other_eid == eid {
            // You can't collide with your own trigger, idiot
            continue;
        }
        let my_tran_n_angle = gtran.tran_n_angle();
        let (_, other_gtran) = shared_data.get(other_eid).unwrap();
        let rhs_tran_n_angle = other_gtran.tran_n_angle();
        let Some((_, cp)) = rx.bounds.bounce_off(
            my_tran_n_angle,
            (&other_tx.bounds, rhs_tran_n_angle.0, rhs_tran_n_angle.1),
        ) else {
            // These things don't overlap, nothing to do
            continue;
        };
        // Create collision records (NOTE: It's symmetric, one for each, and we don't dup)
        if !dup_set.contains(&(eid, other_eid)) {
            let my_collision_record = TriggerCollisionRecord {
                pos: cp,
                my_role: TriggerCollisionRole::Rx,
                rx_eid: eid,
                rx_kind: rx.kind.clone(),
                tx_eid: other_eid,
                tx_kind: other_tx.kind.clone(),
            };
            let my_collision_eid = commands
                .spawn(TriggerCollisionBundle::new(my_collision_record))
                .set_parent(collision_root.eid())
                .id();
            rx.collisions.push_back(my_collision_eid);
            let other_collision_record = TriggerCollisionRecord {
                pos: cp,
                my_role: TriggerCollisionRole::Tx,
                rx_eid: eid,
                rx_kind: rx.kind.clone(),
                tx_eid: other_eid,
                tx_kind: other_tx.kind.clone(),
            };
            let other_collision_eid = commands
                .spawn(TriggerCollisionBundle::new(other_collision_record))
                .set_parent(collision_root.eid())
                .id();
            other_tx.collisions.push_back(other_collision_eid);
            dup_set.insert((eid, other_eid));
        }
    }
}

/// Handles moving all unstuck dynos that have _either_ a staticreceiver or a triggerreceiver
fn move_unstuck_static_or_trigger_receivers(
    bullet_time: Res<BulletTime>,
    relevant_eids: Query<
        Entity,
        (
            Or<(With<StaticRx>, With<TriggerRx>)>,
            Without<Stuck>,
            Or<(With<DynoTran>, With<DynoRot>)>,
            With<PhysicsInitialized>,
        ),
    >,
    shared_data: Query<(Entity, &GlobalTransform)>,
    mut dyno_data: Query<
        (
            Entity,
            Option<&mut DynoTran>,
            Option<&mut DynoRot>,
            &mut Transform,
            Option<&DynoAwareParticleSpawner>,
        ),
        Or<(With<DynoTran>, With<DynoRot>)>,
    >,
    mut static_data: Query<(Entity, &mut StaticRx), Without<Stuck>>,
    mut trigger_txs: Query<(Entity, &mut TriggerTx)>,
    mut trigger_rxs: Query<(Entity, &mut TriggerRx)>,
    mut static_txs: Query<(Entity, &mut StaticTx, &GlobalTransform)>,
    mut commands: Commands,
    collision_root: Res<CollisionRoot>,
    proot: Res<ParticlesRoot>,
) {
    for eid in &relevant_eids {
        // Shared data (immutable)
        let (_, my_gtran) = shared_data.get(eid).unwrap();
        let my_gtran = my_gtran.clone();

        // Mutable dyno data (need to mutate and then assign at end)
        let (_, my_dyno_tran, my_dyno_rot, my_tran, particle_spawner) = dyno_data.get(eid).unwrap();
        let mut my_dyno_tran = my_dyno_tran.map(|inner| inner.clone());
        let mut my_dyno_rot = my_dyno_rot.map(|inner| inner.clone());
        let mut my_tran = my_tran.clone();
        let my_gtran_offset = my_gtran.translation().truncate() - my_tran.translation.truncate();

        // Mutable static data (need to mutate and then assign at end)
        let mut my_static = static_data.get(eid).ok().map(|inner| inner.1.clone());

        // Mutable trigger data (need to mutate and then assign at end)
        let mut my_trigger_rx = trigger_rxs.get(eid).ok().map(|inner| inner.1.clone());
        let mut dup_set = HashSet::<(Entity, Entity)>::new();

        // If we have rotational movement, rotate first
        if let Some(my_dyno_rot) = my_dyno_rot.as_mut() {
            my_tran.rotate_z(my_dyno_rot.rot * bullet_time.delta_seconds());
        }

        // If we have translational movement, inch along
        if let Some(mut my_dyno_tran) = my_dyno_tran.as_mut() {
            let mut amount_moved = 0.0;
            let mut total_to_move = my_dyno_tran.vel.length() * bullet_time.delta_seconds();
            let mut at_least_one_iter = false;
            while !at_least_one_iter || amount_moved < total_to_move {
                at_least_one_iter = true;
                // TODO: This is hella inefficient but I just wanna get it working first
                let dir = my_dyno_tran.vel.normalize_or_zero();
                let mag = (my_dyno_tran.vel.length() * bullet_time.delta_seconds() - amount_moved)
                    .min(MAX_TRAN_STEP_LENGTH);
                let moving = dir * mag;
                my_tran.translation += moving.extend(0.0);
                if let Some(mut my_static_rx) = my_static.as_mut() {
                    resolve_static_collisions(
                        eid,
                        &mut my_static_rx,
                        &mut my_dyno_tran,
                        &mut my_tran,
                        my_gtran_offset,
                        &mut static_txs,
                        &mut commands,
                        &collision_root,
                    );
                }
                // Basically because GlobalTransform doesn't update mid-system we need to do this shenanigans
                let mut mid_step_gtran = my_tran.clone();
                mid_step_gtran.translation += my_gtran_offset.extend(0.0);
                if let Some(my_trigger_rx) = my_trigger_rx.as_mut() {
                    resolve_trigger_collisions(
                        eid,
                        my_trigger_rx,
                        &mid_step_gtran,
                        &shared_data,
                        &mut trigger_txs,
                        &mut commands,
                        &collision_root,
                        &mut dup_set,
                    );
                }
                // If we have a physics-based particle spawner, do something!
                if let Some(particle_spawner) = particle_spawner {
                    particle_spawner.do_spawn(
                        mid_step_gtran.translation.truncate(),
                        &mut commands,
                        &proot,
                    );
                }
                // Update the loop stuff
                amount_moved += MAX_TRAN_STEP_LENGTH;
                total_to_move =
                    total_to_move.min(my_dyno_tran.vel.length() * bullet_time.delta_seconds());
            }
        } else {
            // We're not translating, resolve triggers once to be sure;
            if let Some(my_trigger_rx) = my_trigger_rx.as_mut() {
                // Basically because GlobalTransform doesn't update mid-system we need to do this shenanigans
                let mut mid_step_gtran = my_tran.clone();
                mid_step_gtran.translation += my_gtran_offset.extend(0.0);
                resolve_trigger_collisions(
                    eid,
                    my_trigger_rx,
                    &mid_step_gtran,
                    &shared_data,
                    &mut trigger_txs,
                    &mut commands,
                    &collision_root,
                    &mut dup_set,
                );
            }
        }

        let (_, reset_dyno_tran, reset_dyno_rot, mut reset_tran, _) =
            dyno_data.get_mut(eid).unwrap();
        if let Some(mut reset_dyno_tran) = reset_dyno_tran {
            *reset_dyno_tran = my_dyno_tran.unwrap();
        }
        if let Some(mut reset_dyno_rot) = reset_dyno_rot {
            *reset_dyno_rot = my_dyno_rot.unwrap();
        }
        *reset_tran = my_tran;

        let reset_rx = static_data.get_mut(eid).ok().map(|inner| inner.1);
        if let Some(mut reset_rx) = reset_rx {
            *reset_rx = my_static.unwrap();
        }

        let reset_rx = trigger_rxs.get_mut(eid).ok().map(|inner| inner.1);
        if let Some(mut reset_rx) = reset_rx {
            *reset_rx = my_trigger_rx.unwrap();
        }
    }
}

/// Moves all dynos (both rot and tran) that receive static collisions and ARE stuck. Some may have triggers!
/// SLIGHT BUG: If there are two triggers that are both stuck, and come into contact while stuck, nothing will happen
/// Should be more than fine for this game but is not a perfect physics engine.
fn move_stuck_static_receiver_dynos(
    mut stuck_dynos: Query<
        (
            Entity,
            &Stuck,
            &mut DynoTran,
            &mut Transform,
            Option<&DynoAwareParticleSpawner>,
        ),
        (
            With<StaticRx>,
            With<DynoTran>,
            Without<DynoRot>,
            Without<StaticTx>,
            With<PhysicsInitialized>,
        ),
    >,
    static_providers: Query<&GlobalTransform, With<StaticTx>>,
    mut commands: Commands,
    proot: Res<ParticlesRoot>,
) {
    // First move the things
    for (_eid, stuck, mut dyno_tran, mut tran, particle_spawner) in &mut stuck_dynos {
        let Ok(provider_gtran) = static_providers.get(stuck.parent) else {
            continue;
        };
        dyno_tran.vel = Vec2::ZERO;
        let (provider_tran, provider_angle) = provider_gtran.tran_n_angle();
        let angle_diff = provider_angle - stuck.parent_initial_angle;
        tran.set_angle(stuck.my_initial_angle + angle_diff);
        let rotated_pos = stuck.initial_offset.my_rotate(angle_diff);
        tran.translation.x = provider_tran.x + rotated_pos.x;
        tran.translation.y = provider_tran.y + rotated_pos.y;
        // If we have a physics-based particle spawner, do something!
        if let Some(particle_spawner) = particle_spawner {
            particle_spawner.do_spawn(tran.translation.truncate(), &mut commands, &proot);
        }
    }
}

fn apply_room_wrap(
    mut ents: Query<(&mut Transform, &GlobalTransform), With<RoomWrap>>,
    room_state: Res<State<RoomState>>,
) {
    let room_state = room_state.get();
    for (mut tran, gtran) in &mut ents {
        let half_room_size = room_state.room_size.as_vec2() / 2.0;
        let wrapped_x = (gtran.translation().x + half_room_size.x)
            .rem_euclid(half_room_size.x * 2.0)
            - half_room_size.x;
        let wrapped_y = (gtran.translation().y + half_room_size.y)
            .rem_euclid(half_room_size.y * 2.0)
            - half_room_size.y;
        tran.translation.x += wrapped_x - gtran.translation().x;
        tran.translation.y += wrapped_y - gtran.translation().y;
    }
}

pub(super) fn register_logic(app: &mut App) {
    // Reset collisions during preupdate
    app.add_systems(
        FixedPreUpdate,
        reset_collision_records
            .in_set(PhysicsSet)
            .run_if(in_state(PhysicsState::Active)),
    );
    // Enforce invariants during update when in dev mode
    app.add_systems(
        FixedUpdate,
        enforce_invariants
            .in_set(PhysicsSet)
            .run_if(in_state(PhysicsState::Active))
            .run_if(in_state(AppMode::Dev)),
    );
    // Systems for detecting and resolving collisions
    app.add_systems(
        FixedUpdate,
        (
            initialize_physics,
            move_uninteresting_dynos,
            move_static_provider_dynos,
            move_unstuck_static_or_trigger_receivers,
            move_stuck_static_receiver_dynos,
        )
            .in_set(CollisionsSet)
            .in_set(PhysicsSet)
            .after(InputSet)
            .run_if(in_state(PhysicsState::Active)),
    );
    // Apply room wrap
    app.add_systems(
        FixedUpdate,
        apply_room_wrap
            .in_set(PhysicsSet)
            .after(CollisionsSet)
            .run_if(in_state(MetaStateKind::Room)),
    );
}
