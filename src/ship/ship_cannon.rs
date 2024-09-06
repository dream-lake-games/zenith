use crate::prelude::*;

#[derive(Debug, Clone, Copy, Reflect)]
pub enum ShipCannonKind {
    Laser,
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub(super) struct ShipCannon {
    kind: ShipCannonKind,
    grot: f32,
}
impl ShipCannon {
    pub(super) fn new() -> Self {
        Self {
            kind: ShipCannonKind::Laser,
            grot: 0.0,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub(super) struct ShipAmmo {
    current: f32,
    max: f32,
}
impl ShipAmmo {
    pub(super) fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct ShipFiring {
    pub ttl: Option<f32>,
}

#[derive(Debug, Clone, Component, Reflect)]
struct VisualShipCannon;

#[derive(Bundle)]
struct VisualShipCannonBundle {
    name: Name,
    ship_cannon: VisualShipCannon,
    spatial: SpatialBundle,
    animation_gun: AnimationManager<AnimationShipCannon>,
}
impl VisualShipCannonBundle {
    pub fn new() -> Self {
        Self {
            name: Name::new("ship_cannon"),
            ship_cannon: VisualShipCannon,
            spatial: default(),
            animation_gun: AnimationManager::new(),
        }
    }
}

fn start_ship_firing(
    drag_input: Res<DragInput>,
    mut commands: Commands,
    mut ships: Query<(Entity, &ShipCannon, &mut ShipAmmo), Without<ShipFiring>>,
) {
    if drag_input.get_right_drag_start().is_none() {
        return;
    }
    for (eid, cannon, mut ammo) in &mut ships {
        match cannon.kind {
            ShipCannonKind::Laser => {
                if ammo.current > 0.1 {
                    commands.entity(eid).insert(ShipFiring { ttl: None });
                    ammo.current -= 0.0001;
                }
            }
        }
    }
}

fn spawn_visual_ship_cannons(
    mut commands: Commands,
    ships: Query<(Entity, Option<&Children>), With<Ship>>,
    visual_cannons: Query<Entity, With<VisualShipCannon>>,
) {
    for (eid, ochildren) in &ships {
        let needs_spawn = match ochildren {
            Some(children) => children.iter().all(|cid| !visual_cannons.contains(*cid)),
            None => true,
        };
        if needs_spawn {
            commands
                .spawn(VisualShipCannonBundle::new())
                .set_parent(eid);
        }
    }
}

fn update_ship_cannon_grots(
    drag_input: Res<DragInput>,
    mut ship_cannons: Query<&mut ShipCannon, With<ShipFiring>>,
) {
    let Some(diff) = (match drag_input.get_right_drag_start() {
        Some(start) => {
            let diff = drag_input.get_screen_pos() - start;
            if diff.length_squared() > 0.1 {
                Some(diff)
            } else {
                None
            }
        }
        None => None,
    }) else {
        return;
    };
    for mut ship_cannon in &mut ship_cannons {
        ship_cannon.grot = diff.to_angle();
    }
}

fn update_visual_ship_cannon_rots(
    ship_cannons: Query<(&ShipCannon, &Transform), Without<VisualShipCannon>>,
    mut visuals: Query<(&Parent, &mut Transform), With<VisualShipCannon>>,
) {
    for (parent, mut tran) in &mut visuals {
        let Ok((cannon, ptran)) = ship_cannons.get(parent.get()) else {
            continue;
        };
        let current_angle = ptran.pos_n_angle().1 + tran.pos_n_angle().1;
        let diff = cannon.grot - current_angle;
        let to_set = tran.pos_n_angle().1 + diff;
        tran.set_angle(to_set);
    }
}

pub(super) fn register_ship_cannon(app: &mut App) {
    app.register_type::<ShipCannon>();
    app.register_type::<VisualShipCannon>();
    app.add_systems(
        PostUpdate,
        (
            spawn_visual_ship_cannons,
            update_ship_cannon_grots,
            update_visual_ship_cannon_rots,
        )
            .chain()
            .in_set(ShipSet),
    );
}
