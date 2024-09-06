use bevy::ecs::component::StorageType;

use crate::prelude::*;

#[derive(Resource, Reflect)]
struct ShipBulletConstants {
    distspan: f32,
    explosion_radius: f32,
    bullet_speed: f32,
}
impl Default for ShipBulletConstants {
    fn default() -> Self {
        Self {
            distspan: 1000.0,
            explosion_radius: 10.0,
            bullet_speed: 1000.0,
        }
    }
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct ShipBullet {
    /// The maximum distance the bullet will travel before exploding
    distspan: f32,
    /// Radius of the explosion
    explosion_radius: f32,
}
impl Component for ShipBullet {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let angle = world.get::<Transform>(eid).unwrap().pos_n_angle().1;
            let consts = world.get_resource::<ShipBulletConstants>().unwrap();
            let proper_self = Self {
                distspan: consts.distspan,
                explosion_radius: consts.explosion_radius,
            };
            let vel_addition = Vec2::X.my_rotate(angle) * consts.bullet_speed;
            *world.entity_mut(eid).get_mut::<Self>().unwrap() = proper_self;
            world.entity_mut(eid).get_mut::<DynoTran>().unwrap().vel = vel_addition;
        });
    }
}

#[derive(Component, Debug, Clone, Reflect)]
struct Exploding;

#[derive(Bundle)]
pub struct ShipBulletBundle {
    name: Name,
    ship_bullet: ShipBullet,
    spatial: SpatialBundle,
    dyno_tran: DynoTran,
    static_rx: StaticRx,
    trigger_tx: TriggerTx,
    trigger_rx: TriggerRx,
    room_wrap: RoomWrap,
    animation: AnimationManager<AnimationShipBulletDefault>,
    mirage: MirageAnimationManager,
    dyno_aware: DynoAwareParticleSpawner,
}
impl ShipBulletBundle {
    /// NOTE: Some fucky stuff happens with hooks, some of these values updated/corrected there
    pub fn new(pos: Vec2, parent_vel: Vec2, ang: f32, room_state: &RoomState) -> Self {
        let shape = Shape::Circle {
            center: Vec2::ZERO,
            radius: 4.0,
        };
        let mut all_shapes = vec![shape.clone()];
        for offset in room_state.mirage_offsets() {
            all_shapes.push(shape.clone().with_offset(offset));
        }
        let mut spatial = spat_tran!(pos.x, pos.y, ZIX_SHIP);
        spatial.transform.rotate_z(ang);
        let particle = Particle::new(Vec2::ZERO)
            .with_colors(
                Color::srgb_u8(193, 102, 69),
                Color::srgba_u8(255, 255, 255, 0),
            )
            .with_sizes(4.0, 2.0);
        Self {
            name: Name::new("ship_bullet"),
            ship_bullet: ShipBullet::default(),
            spatial,
            dyno_tran: DynoTran { vel: parent_vel },
            static_rx: StaticRx::from_kind_n_shape(StaticRxKind::Stop, shape.clone()),
            trigger_tx: TriggerTx::from_kind_n_shapes(TriggerKind::ShipBullet, all_shapes),
            trigger_rx: TriggerRx::from_kind_n_shape(TriggerKind::ShipBullet, shape),
            room_wrap: RoomWrap,
            animation: AnimationManager::new(),
            mirage: MirageAnimationManager::room_offsets(room_state),
            dyno_aware: DynoAwareParticleSpawner::new(vec![particle]),
        }
    }
}

fn update_ship_bullets(
    mut commands: Commands,
    mut bullets_q: Query<(Entity, &mut ShipBullet, &mut DynoTran)>,
    bullet_time: Res<BulletTime>,
) {
    for (eid, mut ship_bullet, mut dyno_tran) in &mut bullets_q {
        // Bullets explode wh
        if ship_bullet.distspan <= 0.0 || dyno_tran.vel.length_squared() <= 0.1 {
            dyno_tran.vel = Vec2::ZERO;
            commands.entity(eid).insert(Exploding);
        }
        ship_bullet.distspan -= dyno_tran.vel.length() * bullet_time.delta_seconds();
    }
}

pub(super) fn register_ship_bullet(app: &mut App) {
    app.register_type::<ShipBullet>();
    app.insert_resource(ShipBulletConstants::default());
    debug_resource!(app, ShipBulletConstants);

    app.add_systems(Update, update_ship_bullets.after(PhysicsSet));
}
