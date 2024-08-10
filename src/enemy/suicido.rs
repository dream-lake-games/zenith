pub use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
struct Suicido;

#[derive(Bundle)]
pub struct SuicidoBundle {
    name: Name,
    spatial: SpatialBundle,
    static_rx: StaticRx,
    dyno_tran: DynoTran,
    patrol: PatrolWatch<Ship>,
    wrap: RoomWrap,
    animation: AnimationManager<AnimationSuicido>,
    mirage: MirageAnimationManager,
}
impl SuicidoBundle {
    pub fn new(pos: Vec2, room_state: &RoomState) -> Self {
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
            patrol: PatrolWatch::<Ship>::new(Bounds::from_shape(Shape::Circle {
                center: Vec2::ZERO,
                radius: 100.0,
            })),
            wrap: RoomWrap,
            animation: AnimationManager::new(),
            mirage: MirageAnimationManager::room_offsets(room_state),
        }
    }
}

fn update_suicidos(mut commands: Commands, suicidos: Query<(Entity, Option<&PatrolActive>)>) {
    for (eid, patrol) in &suicidos {
        match patrol {
            Some(patrol) => {
                let follow = Follow::new(patrol.target_eid, 60.0, 30.0);
                commands.entity(eid).insert(follow.clone());
            }
            None => {
                commands.entity(eid).remove::<Follow>();
            }
        }
    }
}

pub(super) fn register_suicidos(app: &mut App) {
    app.register_type::<Suicido>();
    app.add_systems(Update, update_suicidos.after(PhysicsSet));
}
