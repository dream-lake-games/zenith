pub use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
struct Suicido;

#[derive(Bundle)]
pub struct SuicidoBundle {
    name: Name,
    spatial: SpatialBundle,
    static_rx: StaticRx,
    dyno_tran: DynoTran,
    follow: Follow,
    wrap: RoomWrap,
    animation: AnimationManager<AnimationSuicido>,
    mirage: MirageAnimationManager,
}
impl SuicidoBundle {
    pub fn new(pos: Vec2, ship_eid: Entity, room_state: &RoomState) -> Self {
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
            follow: Follow::new(ship_eid, 60.0, 60.0).with_acceptable_dist_range((50.0, f32::MAX)),
            wrap: RoomWrap,
            animation: AnimationManager::new(),
            mirage: MirageAnimationManager::room_offsets(room_state),
        }
    }
}
