use bevy::reflect::GetTypeRegistration;

use crate::prelude::*;

pub trait Patrollable:
    Component + std::fmt::Debug + Clone + Reflect + FromReflect + TypePath + GetTypeRegistration
{
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct PatrolWatch<C: Component + std::fmt::Debug + Clone + Reflect> {
    vision: Bounds,
    _ignore: Option<C>,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct PatrolActive {
    pub target_eid: Entity,
    pub time_seen: f32,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct PatrolInactive;

pub fn register_patrol<C: Patrollable>(app: &mut App) {
    app.register_type::<PatrolWatch<C>>();
    app.register_type::<PatrolActive>();
    app.register_type::<PatrolInactive>();
}
