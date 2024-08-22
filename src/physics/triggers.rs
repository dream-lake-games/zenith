use crate::prelude::*;

/// All the different kinds of triggers.
#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    /// Basically marks the hitbox of the protagonist
    Ship,
    /// Bullet fired by the protagonist
    ShipBullet,
}

/// Marks an object as being a trigger provider
#[derive(Component, Debug, Clone, Reflect)]
pub struct TriggerTx {
    pub kind: TriggerKind,
    pub bounds: Bounds,
    pub collisions: VecDeque<Entity>,
}
impl TriggerTx {
    pub fn from_kind_n_shape(kind: TriggerKind, shape: Shape) -> Self {
        Self {
            kind,
            bounds: Bounds::from_shape(shape),
            collisions: default(),
        }
    }

    pub fn from_kind_n_shapes(kind: TriggerKind, shapes: Vec<Shape>) -> Self {
        Self {
            kind,
            bounds: Bounds::from_shapes(shapes),
            collisions: default(),
        }
    }
}

/// Marks an object as being a trigger receiver
#[derive(Component, Debug, Clone, Reflect)]
pub struct TriggerRx {
    pub kind: TriggerKind,
    pub bounds: Bounds,
    pub collisions: VecDeque<Entity>,
}
impl TriggerRx {
    pub fn from_kind_n_shape(kind: TriggerKind, shape: Shape) -> Self {
        Self {
            kind,
            bounds: Bounds::from_shape(shape),
            collisions: default(),
        }
    }

    pub fn from_kind_n_shapes(kind: TriggerKind, shapes: Vec<Shape>) -> Self {
        Self {
            kind,
            bounds: Bounds::from_shapes(shapes),
            collisions: default(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub enum TriggerCollisionRole {
    Tx,
    Rx,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct TriggerCollisionRecord {
    pub my_role: TriggerCollisionRole,
    pub tx_eid: Entity,
    pub tx_kind: TriggerKind,
    pub rx_eid: Entity,
    pub rx_kind: TriggerKind,
}
#[derive(Bundle)]
pub struct TriggerCollisionBundle {
    name: Name,
    record: TriggerCollisionRecord,
}
impl TriggerCollisionBundle {
    pub fn new(record: TriggerCollisionRecord) -> Self {
        Self {
            name: Name::new("trigger_collision"),
            record,
        }
    }
}
