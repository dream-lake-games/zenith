use crate::prelude::*;

mod launch_juice;
pub mod ship_bullet;
pub mod ship_cannon;

pub use ship_bullet::*;
use ship_cannon::*;

#[derive(Resource, Reflect)]
pub struct ShipBaseConstants {
    max_num_launches: u32,
    max_launch_time: f32,
    launch_recharge_time: f32,
    max_num_fires: u32,
    max_fire_time: f32,
    fire_recharge_time: f32,
}
impl Default for ShipBaseConstants {
    fn default() -> Self {
        Self {
            max_num_launches: 1,
            max_launch_time: 0.75,
            launch_recharge_time: 1.0,
            max_num_fires: 3,
            max_fire_time: 0.75,
            fire_recharge_time: 1.0,
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShipSet;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Ship;

#[derive(Component, Debug, Clone, Reflect)]
pub struct ShipTail;

#[derive(Component, Debug, Clone, Reflect)]
pub struct ShipLaunchState {
    /// How many launches the ship currently has
    pub num_launches: u32,
    /// How many total launches can the ship store at once
    pub max_num_launches: u32,
    /// How long can we be in launch-bullet time before we force fire?
    pub max_launch_time: f32,
    /// The current launch (how long we've spent launching), None if not launchingA
    pub current_launch: Option<f32>,
    /// How long does it take the ship to recharge a single launch
    pub launch_recharge_time: f32,
    /// The current recharge timer, None if num_launches = max_num_launches
    pub current_recharge: Option<f32>,
}
impl ShipLaunchState {
    pub fn new(max_num_launches: u32, max_launch_time: f32, launch_recharge_time: f32) -> Self {
        Self {
            num_launches: max_num_launches,
            max_num_launches,
            max_launch_time,
            current_launch: None,
            launch_recharge_time,
            current_recharge: None,
        }
    }
}

/// Put on Ship entity when it's charging a launch. When time_left hit's 0, forced to launch
#[derive(Component, Debug, Clone, Reflect)]
struct ShipLaunching {
    time_left: f32,
}

#[derive(Bundle)]
pub struct ShipBundle {
    name: Name,
    ship: Ship,
    ship_cannon: ShipCannon,
    ship_ammo: ShipAmmo,
    ship_launch_state: ShipLaunchState,
    spatial: SpatialBundle,
    dyno_tran: DynoTran,
    static_rx: StaticRx,
    trigger_tx: TriggerTx,
    trigger_rx: TriggerRx,
    animation_body: AnimationManager<AnimationShipBody>,
    camera_leader: DynamicCameraLeader,
    wrap_room: RoomWrap,
    dyno_particles: DynoAwareParticleSpawner,
}
impl ShipBundle {
    pub fn new(pos: Vec2, room_state: &RoomState, base_consts: &ShipBaseConstants) -> Self {
        let shape = Shape::Circle {
            center: Vec2::ZERO,
            radius: 6.0,
        };
        let mut all_shapes = vec![shape.clone()];
        for offset in room_state.mirage_offsets() {
            all_shapes.push(shape.clone().with_offset(offset));
        }
        Self {
            name: Name::new("ship"),
            ship: Ship,
            ship_cannon: ShipCannon::new(),
            ship_ammo: ShipAmmo::new(3.0),
            spatial: spat_tran!(pos.x, pos.y, ZIX_SHIP),
            ship_launch_state: ShipLaunchState::new(
                base_consts.max_num_launches,
                base_consts.max_launch_time,
                base_consts.launch_recharge_time,
            ),
            dyno_tran: default(),
            static_rx: StaticRx::from_kind_n_shape(
                StaticRxKind::Normal,
                Shape::Circle {
                    center: Vec2::ZERO,
                    radius: 6.0,
                },
            ),
            trigger_tx: TriggerTx::from_kind_n_shapes(TriggerKind::Ship, all_shapes),
            trigger_rx: TriggerRx::from_kind_n_shape(TriggerKind::Ship, shape),
            animation_body: AnimationManager::new(),
            camera_leader: DynamicCameraLeader,
            wrap_room: RoomWrap,
            dyno_particles: DynoAwareParticleSpawner::new(vec![Particle::new(Vec2::ZERO)
                .with_colors(
                    tailwind::GREEN_300.into(),
                    Srgba::new(0.0, 1.0, 0.0, 0.0).into(),
                )
                .with_sizes(4.0, 2.0)
                .with_lifespan(0.75)])
            .with_poses(vec![Vec2::ZERO]),
        }
    }
}

#[derive(Bundle)]
struct ShipTailBundle {
    name: Name,
    ship_tail: ShipTail,
    spatial: SpatialBundle,
    animation_tail: AnimationManager<AnimationShipTail>,
}
impl ShipTailBundle {
    pub fn new() -> Self {
        Self {
            name: Name::new("ship_tail"),
            ship_tail: ShipTail,
            spatial: default(),
            animation_tail: AnimationManager::new(),
        }
    }
}

/// Spawns the tail and gun on ships without them
fn spawn_ship_parts(
    mut commands: Commands,
    ship: Query<(Entity, Option<&Children>), With<Ship>>,
    tails: Query<Entity, With<ShipTail>>,
) {
    for (eid, ochildren) in &ship {
        let spawn_gun;
        let spawn_tail;
        match ochildren {
            Some(children) => {
                spawn_tail = !children.iter().any(|cid| tails.contains(*cid));
            }
            None => {
                spawn_gun = true;
                spawn_tail = true;
            }
        }
        if spawn_tail {
            commands.spawn(ShipTailBundle::new()).set_parent(eid);
        }
    }
}

/// Handles starting a `current_launch`, ending, and recharging
fn update_ship_launch(
    mut commands: Commands,
    drag_input: Res<DragInput>,
    mut ships: Query<(Entity, &mut ShipLaunchState, &mut DynoTran, &mut Transform), With<Ship>>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut launches: EventReader<Launch>,
    mut force_launch: EventWriter<ForceLaunch>,
) {
    // First handle launching
    for (eid, mut launch_state, mut dyno_tran, mut tran) in &mut ships {
        match launch_state.current_launch {
            Some(launch_time) => {
                if launch_time < launch_state.max_launch_time {
                    // Continue charging the launch, no need to force fire
                    launch_state.current_launch = Some(launch_time + time.delta_seconds());
                } else {
                    // Force fire
                    // TODO: maybe? I am too lazy to fully think through whether there could be a bug here with these events.
                    // Intuition tells me that even if there is, it's only a one frame window and you'd have to initiate two launches
                    // so not a big deal.
                    force_launch.send(ForceLaunch);
                }
                if let Some(launch) = launches.read().last() {
                    // DO THE LAUNCH
                    commands.entity(eid).remove::<Stuck>();
                    dyno_tran.vel = launch.0;
                    if launch.0.length_squared() > 0.1 {
                        tran.set_angle(launch.0.to_angle());
                    }
                    launch_state.current_launch = None;
                    let mut shrink_tran = tran.clone();
                    shrink_tran.translation.z -= 1.0;
                    commands.spawn(RingShrink::new(tran.clone()));
                }
            }
            None => {
                if launch_state.num_launches > 0 && drag_input.get_left_drag_start().is_some() {
                    launch_state.current_launch = Some(0.0);
                    launch_state.num_launches -= 1;
                }
            }
        }
    }
    // Then handle recharging
    for (_, mut launch_state, _, _) in &mut ships {
        match launch_state.current_recharge {
            Some(recharge_time) => {
                if recharge_time > launch_state.launch_recharge_time {
                    launch_state.num_launches = launch_state
                        .max_num_launches
                        .min(launch_state.num_launches + 1);
                    launch_state.current_recharge = None;
                } else {
                    launch_state.current_recharge =
                        Some(recharge_time + bullet_time.delta_seconds());
                }
            }
            None => {
                if launch_state.num_launches < launch_state.max_num_launches {
                    launch_state.current_recharge = Some(0.0);
                }
            }
        }
    }
}

pub(super) struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ship>();
        app.register_type::<ShipLaunchState>();
        app.register_type::<ShipTail>();

        app.add_systems(Update, (update_ship_launch).in_set(ShipSet));

        app.add_systems(
            PostUpdate,
            spawn_ship_parts.in_set(ShipSet).before(AnimationSet),
        );

        app.insert_resource(ShipBaseConstants::default());
        debug_resource!(app, ShipBaseConstants);

        launch_juice::register_launch_juice(app);
        ship_bullet::register_ship_bullet(app);
        ship_cannon::register_ship_cannon(app);
    }
}
