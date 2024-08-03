use crate::prelude::*;

/// All the different kinds of triggers.
#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    /// Basically marks the hitbox of the protagonist
    Ship,
}

/// Marks an object as being a "triggerable" physics object.
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
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct TriggerCollisionRecord {
    pub pos: Vec2,
    pub other_eid: Entity,
    pub other_kind: TriggerKind,
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
