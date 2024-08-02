use bevy::sprite::Material2dPlugin;

use crate::prelude::*;

pub mod manager;
pub(self) mod mat;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationSet;

pub(super) struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<mat::AnimationMaterial>::default());
    }
}
