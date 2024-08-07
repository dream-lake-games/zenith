use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct Ship;

#[derive(Bundle)]
pub struct ShipBundle {
    name: Name,
    ship: Ship,
    spatial: SpatialBundle,
    dyno_tran: DynoTran,
    static_rx: StaticRx,
    trigger_rx: TriggerRx,
    animation: AnimationManager<AnimationShip>,
    camera_leader: DynamicCameraLeader,
    wrap_room: RoomWrap,
}
impl ShipBundle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            name: Name::new("ship"),
            spatial: spat_tran!(pos.x, pos.y, ZIX_SHIP),
            ship: Ship,
            dyno_tran: default(),
            static_rx: StaticRx::from_kind_n_shape(
                StaticRxKind::Normal,
                Shape::Circle {
                    center: Vec2::ZERO,
                    radius: 6.0,
                },
            ),
            trigger_rx: TriggerRx::from_kind_n_shape(
                TriggerKind::Ship,
                Shape::Circle {
                    center: Vec2::ZERO,
                    radius: 6.0,
                },
            ),
            animation: AnimationManager::new(),
            camera_leader: DynamicCameraLeader,
            wrap_room: RoomWrap,
        }
    }
}
