pub use crate::prelude::*;

#[derive(Resource, Reflect)]
struct SuicidoConstants {
    accel: f32,
    max_speed: f32,
    drag: f32,
}
impl Default for SuicidoConstants {
    fn default() -> Self {
        Self {
            accel: 240.0,
            max_speed: 60.0,
            drag: 0.95,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
struct Suicido;

#[derive(Bundle)]
pub struct SuicidoBundle {
    name: Name,
    spatial: SpatialBundle,
    static_rx: StaticRx,
    dyno_tran: DynoTran,
    wrap: RoomWrap,
    animation: AnimationManager<AnimationSuicido>,
    mirage: MirageAnimationManager,
    follow: Follow,
}
impl SuicidoBundle {
    pub fn new(pos: Vec2, follow_eid: Entity, room_state: &RoomState) -> Self {
        let default_consts = SuicidoConstants::default();
        Self {
            name: Name::new("suicido"),
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
            follow: Follow::new(follow_eid, default_consts.accel, default_consts.max_speed)
                .with_look_at_target(true),
        }
    }
}

fn update_suicidos(
    mut suicidos: Query<(&mut Follow, &mut DynoTran)>,
    constants: Res<SuicidoConstants>,
) {
    for (mut follow, mut dyno_tran) in &mut suicidos {
        if constants.is_added() || constants.is_changed() {
            follow.set_accel(constants.accel);
            follow.set_max_speed(constants.max_speed);
        }
        dyno_tran.vel *= constants.drag;
    }
}

pub(super) fn register_suicidos(app: &mut App) {
    app.register_type::<Suicido>();
    app.add_systems(Update, update_suicidos.after(PhysicsSet));
    app.insert_resource(SuicidoConstants::default());
    debug_resource!(app, SuicidoConstants);
}
