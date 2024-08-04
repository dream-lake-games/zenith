use crate::prelude::*;

/// Anything that needs to move translationally in the world. Can be either triggers or statics.
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct DynoTran {
    pub vel: Vec2,
}

/// Anything that needs to move rotationally in the world. Can be either triggers or statics.
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct DynoRot {
    pub rot: f32,
}

/// Anything that needs to behave like it moves infinitely, wrapping at room boundaries
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct RoomWrap;
