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
    trigger_tx: TriggerTx,
    trigger_rx: TriggerRx,
    animation: AnimationManager<AnimationShip>,
    camera_leader: DynamicCameraLeader,
    wrap_room: RoomWrap,
}
impl ShipBundle {
    pub fn new(pos: Vec2, room_state: &RoomState) -> Self {
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
            trigger_tx: TriggerTx::from_kind_n_shapes(TriggerKind::Ship, all_shapes),
            trigger_rx: TriggerRx::from_kind_n_shape(TriggerKind::Ship, shape),
            animation: AnimationManager::new(),
            camera_leader: DynamicCameraLeader,
            wrap_room: RoomWrap,
        }
    }
}
