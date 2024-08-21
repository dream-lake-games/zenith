pub use crate::prelude::*;

#[derive(Resource, Reflect)]
struct SuicidoConstants {
    rot_speed: f32,
    charge_drag: f32,
    charge_time: f32,
    wander_launch_speed: f32,
    engaged_launch_speed: f32,
    max_launch_time: f32,
    explode_vision_box: Vec2,
    prefer_fut: f32,
    moving_away_mult: f32,
}
impl Default for SuicidoConstants {
    fn default() -> Self {
        Self {
            rot_speed: PI * 2.0,
            charge_drag: 0.95,
            charge_time: 1.0,
            wander_launch_speed: 50.0,
            engaged_launch_speed: 90.0,
            max_launch_time: 2.0,
            explode_vision_box: Vec2::new(36.0, 30.0),
            prefer_fut: 0.5,
            moving_away_mult: 3.0,
        }
    }
}

#[derive(Debug, Clone, Reflect)]
enum ChargeGoal {
    Angle { speed: f32 },
    Entity { eid: Entity },
}

#[derive(Component, Debug, Clone, Reflect)]
struct Charging {
    goal: ChargeGoal,
    time: f32,
}
impl Default for Charging {
    fn default() -> Self {
        Self {
            goal: ChargeGoal::Angle {
                speed: thread_rng().gen_range(-0.5..0.5),
            },
            time: 0.0,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
struct Launching {
    dist_to_goal_sq: Option<(f32, Entity)>,
    time: f32,
}

#[derive(Component, Debug, Clone, Reflect)]
struct Exploding {
    has_spawned_circle: bool,
}

#[derive(Debug, Clone, Reflect, Component)]
struct Suicido;

#[derive(Component, Debug, Clone, Reflect)]
struct EngageVision;
impl Patrollable for EngageVision {}

#[derive(Component, Debug, Clone, Reflect)]
struct ExplodeVision;
impl Patrollable for ExplodeVision {}

#[derive(Bundle)]
pub struct SuicidoBundle {
    name: Name,
    suicido: Suicido,
    spatial: SpatialBundle,
    static_rx: StaticRx,
    dyno_tran: DynoTran,
    wrap: RoomWrap,
    animation: AnimationManager<AnimationSuicidoBody>,
    mirage: MirageAnimationManager,
    engage: PatrolWatch<Ship, EngageVision>,
    charging: Charging,
    dyno_particles: DynoAwareParticleSpawner,
}
impl SuicidoBundle {
    pub fn new(pos: Vec2, room_state: &RoomState) -> Self {
        Self {
            name: Name::new("suicido"),
            suicido: Suicido,
            spatial: spat_tran!(pos.x, pos.y, ZIX_ENEMY),
            static_rx: StaticRx::from_kind_n_shape(
                StaticRxKind::Normal,
                Shape::Circle {
                    center: Vec2::ZERO,
                    radius: 6.0,
                },
            ),
            dyno_tran: default(),
            wrap: RoomWrap,
            animation: AnimationManager::new(),
            mirage: MirageAnimationManager::room_offsets(room_state),
            engage: PatrolWatch::new(Bounds::from_shape(Shape::Circle {
                center: Vec2::ZERO,
                radius: 100.0,
            })),
            charging: Charging::default(),
            dyno_particles: DynoAwareParticleSpawner::new(vec![Particle::new(Vec2::ZERO)
                .with_colors(
                    tailwind::PINK_600.into(),
                    Srgba::new(0.0, 0.0, 0.0, 0.0).into(),
                )
                .with_sizes(4.0, 2.0)
                .with_lifespan(0.5)]),
        }
    }
}

fn debug_suicidos() {}

/// So we don't have to specify the explode vision on spawn (and leak visibility of constants)
/// do it here
fn attach_explode_vision(
    mut commands: Commands,
    relevant: Query<Entity, (With<Suicido>, Without<PatrolWatch<Ship, ExplodeVision>>)>,
    constants: Res<SuicidoConstants>,
) {
    for eid in &relevant {
        commands
            .entity(eid)
            .insert(PatrolWatch::<Ship, ExplodeVision>::new(Bounds::from_shape(
                Shape::Polygon {
                    points: simple_rect_offset(
                        constants.explode_vision_box.x,
                        constants.explode_vision_box.y,
                        Vec2::new(constants.explode_vision_box.x / 2.0, 0.0),
                    ),
                },
            )));
    }
}

/// Slows down suicidos which are charging
/// NOTE: Needs to be separate from update because it runs on diff schedule
fn drag_charging_suicidos(
    mut suicidos: Query<&mut DynoTran, (With<Suicido>, With<Charging>)>,
    constants: Res<SuicidoConstants>,
) {
    for mut dyno_tran in &mut suicidos {
        dyno_tran.vel *= constants.charge_drag;
    }
}

/// Aims the suicidos and transitions to launch when timer is done
fn update_charging_suicidos(
    mut commands: Commands,
    mut suicidos: Query<
        (
            Entity,
            &mut Charging,
            &mut DynoTran,
            &mut Transform,
            &GlobalTransform,
            Option<&PatrolActive<EngageVision>>,
            Option<&mut SimpleParticleSpawner>,
        ),
        With<Suicido>,
    >,
    ship_poses: Query<(&GlobalTransform, &DynoTran), (With<Ship>, Without<Suicido>)>,
    constants: Res<SuicidoConstants>,
    bullet_time: Res<BulletTime>,
    meta_state: Res<State<MetaState>>,
) {
    let wrap_size = meta_state
        .get_room_state()
        .map(|r| r.room_size)
        .unwrap_or(IDEAL_VEC)
        .as_vec2();
    for (eid, mut charging, mut dyno_tran, mut tran, gtran, engaged, mut spawner) in &mut suicidos {
        // Update the charge goal if needed
        match charging.goal.clone() {
            ChargeGoal::Angle { .. } => {
                if let Some(active) = engaged {
                    charging.goal = ChargeGoal::Entity {
                        eid: active.target_eid,
                    };
                }
            }
            ChargeGoal::Entity { eid } => {
                if !ship_poses.contains(eid) {
                    charging.goal = ChargeGoal::Angle { speed: 0.0 };
                }
            }
        }
        // Handle rotation
        let (my_pos, my_ang) = gtran.pos_n_angle();
        let (ang_diff, dist_to_goal_sq) = match charging.goal {
            ChargeGoal::Angle { speed } => (speed, None),
            ChargeGoal::Entity { eid: goal_eid } => {
                let (goal_pos, goal_dyno) = ship_poses.get(goal_eid).unwrap();
                let (mut goal_pos, _) = goal_pos.pos_n_angle();
                goal_pos += constants.prefer_fut * goal_dyno.vel;
                let diff = room_diff(goal_pos, my_pos, wrap_size);
                let goal_ang = diff.to_angle();
                (
                    shortest_rotation(my_ang, goal_ang),
                    Some((diff.length_squared(), goal_eid)),
                )
            }
        };
        let ang_diff = ang_diff.signum()
            * ((bullet_time.delta_seconds() * constants.rot_speed).min(ang_diff.abs()));
        tran.rotate_z(ang_diff);
        // Update timing, potentially launch
        charging.time += bullet_time.delta_seconds();
        if charging.time > constants.charge_time {
            commands.entity(eid).remove::<Charging>();
            commands.entity(eid).insert(Launching {
                dist_to_goal_sq: dist_to_goal_sq.map(|(_, b)| (f32::MAX, b)),
                time: 0.0,
            });
            let speed = if dist_to_goal_sq.is_some() {
                commands.spawn(RingShrink::new(tran.clone()));
                constants.engaged_launch_speed
            } else {
                constants.wander_launch_speed
            };
            dyno_tran.vel = Vec2::X.my_rotate(my_ang) * speed;
        }
        // Update particle spawner
        if let Some(spawner) = spawner.as_mut() {
            let frac = charging.time / constants.charge_time;
            spawner.references[0].internal.start_color = Srgba::new(
                1.0,
                (130.0 / 255.0) * (1.0 - frac).powi(2),
                (150.0 / 255.0) * (1.0 - frac).powi(2),
                1.0,
            );
            let vel = Vec2::X.my_rotate(gtran.pos_n_angle().1 + PI) * (40.0 + 160.0 * frac * frac);
            for reference in spawner.references.iter_mut() {
                reference.vel = Some(vel);
            }
        }
    }
}

/// Watches for an explosion, times the launch to transition to charge after certain amount of time
/// Also watches for patrol vision becoming active, or getting further away from target, both of which
/// will trigger charging again
fn update_launching_suicidos(
    mut commands: Commands,
    mut suicidos: Query<
        (
            Entity,
            &mut Launching,
            &GlobalTransform,
            Option<&PatrolActive<EngageVision>>,
            Option<&PatrolActive<ExplodeVision>>,
        ),
        With<Suicido>,
    >,
    ship_poses: Query<&GlobalTransform, With<Ship>>,
    constants: Res<SuicidoConstants>,
    bullet_time: Res<BulletTime>,
    meta_state: Res<State<MetaState>>,
) {
    let wrap_size = meta_state.wrap_size();
    for (eid, mut launching, gtran, engaged, explode_range) in &mut suicidos {
        if explode_range.is_some() {
            commands.entity(eid).insert(Exploding {
                has_spawned_circle: false,
            });
            continue;
        }
        let (my_pos, _) = gtran.pos_n_angle();
        // Update distance to goal
        let new_dist_to_goal_sq = match engaged {
            Some(active) => {
                let (goal_pos, _) = ship_poses.get(active.target_eid).unwrap().pos_n_angle();
                let diff = room_diff(goal_pos, my_pos, wrap_size);
                Some((diff.length_squared(), active.target_eid))
            }
            None => None,
        };
        let moving_away = match (launching.dist_to_goal_sq, new_dist_to_goal_sq) {
            // Target is seen in both cases, but is getting further away
            (Some((old_dist, _)), Some((new_dist, new_eid))) => {
                if new_dist > old_dist + 0.2 {
                    Some(new_eid)
                } else {
                    None
                }
            }
            // Target was seen last frame but not this frame, charge towards it
            (Some((_, old_eid)), None) => Some(old_eid),
            // Target never seen
            _ => None,
        };
        launching.dist_to_goal_sq = new_dist_to_goal_sq;
        // Update timing
        let time_mult = if moving_away.is_some() {
            constants.moving_away_mult
        } else {
            1.0
        };
        launching.time += time_mult * bullet_time.delta_seconds();
        // Charge if we're moving away OR out of time
        if launching.time > constants.max_launch_time {
            let goal = match engaged {
                Some(active) => ChargeGoal::Entity {
                    eid: active.target_eid,
                },
                None => moving_away
                    .map(|last_eid| ChargeGoal::Entity { eid: last_eid })
                    .unwrap_or(ChargeGoal::Angle {
                        speed: thread_rng().gen_range(
                            (-constants.rot_speed / 100.0)..(constants.rot_speed / 100.0),
                        ),
                    }),
            };
            commands.entity(eid).remove::<Launching>();
            commands.entity(eid).insert(Charging { goal, time: 0.0 });
        }
    }
}

fn update_exploding_suicidos(
    mut commands: Commands,
    mut suicidos: Query<
        (
            Entity,
            &mut Exploding,
            &GlobalTransform,
            &DynoTran,
            &AnimationManager<AnimationSuicidoBody>,
            &AnimationBodyProgress<AnimationSuicidoBody>,
        ),
        With<Suicido>,
    >,
    meta_state: Res<State<MetaState>>,
) {
    for (eid, mut exploding, gtran, dyno_tran, animation_state, animation_progress) in &mut suicidos
    {
        // Spawn the explosion circle
        commands
            .entity(eid)
            .remove::<PatrolWatch<Ship, EngageVision>>();
        commands
            .entity(eid)
            .remove::<PatrolWatch<Ship, ExplodeVision>>();
        commands.entity(eid).remove::<Charging>();
        commands.entity(eid).remove::<Launching>();
        if !exploding.has_spawned_circle {
            if animation_state.get_state() == AnimationSuicidoBody::Explode
                && animation_progress.get_body_ix(AnimationBody_AnimationSuicidoBody::explode)
                    == Some(3)
            {
                exploding.has_spawned_circle = true;
                commands.spawn(ExplosionCircleBundle::new(
                    gtran.pos_n_angle().0,
                    dyno_tran,
                    &meta_state.get_room_state().unwrap(),
                ));
            }
        }
    }
}

fn update_suicido_animations(
    mut suicidos: Query<(
        &mut AnimationManager<AnimationSuicidoBody>,
        Option<&Charging>,
        Option<&Launching>,
        Option<&Exploding>,
    )>,
) {
    for (mut manager, charging, launching, exploding) in &mut suicidos {
        match (charging, launching, exploding) {
            (Some(_), None, None) => manager.set_state(AnimationSuicidoBody::Charge),
            (None, Some(_), None) => manager.set_state(AnimationSuicidoBody::Launch),
            (None, None, Some(_)) => manager.set_state(AnimationSuicidoBody::Explode),
            _ => manager.set_state(AnimationSuicidoBody::Charge),
        }
    }
}

fn add_or_remove_particle_spawners(
    mut commands: Commands,
    add_to: Query<Entity, (With<Suicido>, Added<Charging>)>,
    remove_from: Query<Entity, (With<Suicido>, Without<Charging>)>,
    remove_dyno_aware: Query<Entity, (With<Suicido>, Added<Exploding>)>,
) {
    let core_particle = Particle::new(Vec2::ZERO)
        .with_colors(
            Srgba::rgb_u8(255, 130, 150).into(),
            Srgba::new(1.0, 1.0, 1.0, 0.0).into(),
        )
        .with_lifespan(0.25)
        .with_sizes(7.0, 2.0);
    let light_particle = Particle::new(Vec2::ZERO)
        .with_colors(Color::WHITE, Srgba::new(1.0, 1.0, 1.0, 0.0).into())
        .with_lifespan(0.17)
        .with_sizes(7.0, 2.0)
        .with_render_layers(LightLayer::render_layers());
    for eid in &add_to {
        commands.entity(eid).insert(SimpleParticleSpawner::new(vec![
            core_particle.clone(),
            light_particle.clone(),
        ]));
    }
    for eid in &remove_from {
        commands.entity(eid).remove::<SimpleParticleSpawner>();
    }
    for eid in &remove_dyno_aware {
        commands.entity(eid).remove::<DynoAwareParticleSpawner>();
    }
}

#[derive(Component, Debug, Clone, Reflect)]
struct ExplosionCircle;

#[derive(Bundle)]
struct ExplosionCircleBundle {
    name: Name,
    explosion_circle: ExplosionCircle,
    spatial: SpatialBundle,
    dyno_tran: DynoTran,
    wrap: RoomWrap,
    animation: AnimationManager<AnimationSuicidoExplosionCircle>,
    mirage: MirageAnimationManager,
}
impl ExplosionCircleBundle {
    fn new(pos: Vec2, dyno_tran: &DynoTran, room_state: &RoomState) -> Self {
        Self {
            name: Name::new("explosion_circle"),
            explosion_circle: ExplosionCircle,
            spatial: spat_tran!(pos.x, pos.y, ZIX_ENEMY),
            dyno_tran: dyno_tran.clone(),
            wrap: RoomWrap,
            animation: AnimationManager::new(),
            mirage: MirageAnimationManager::room_offsets(room_state),
        }
    }
}

pub(super) fn register_suicidos(app: &mut App) {
    app.register_type::<Suicido>();
    app.add_systems(
        Update,
        (
            debug_suicidos,
            attach_explode_vision,
            update_charging_suicidos,
            update_launching_suicidos,
            update_exploding_suicidos,
            update_suicido_animations,
            add_or_remove_particle_spawners,
        )
            .after(PhysicsSet),
    );
    app.add_systems(BulletUpdate, drag_charging_suicidos);
    app.insert_resource(SuicidoConstants::default());
    debug_resource!(app, SuicidoConstants);
    register_patrol::<Ship, EngageVision>(app);
    register_patrol::<Ship, ExplodeVision>(app);
}
