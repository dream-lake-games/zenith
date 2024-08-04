use bevy::sprite::Material2dPlugin;

use crate::prelude::*;

pub mod animation_manager;
pub mod defns;
pub mod macros;
pub(self) mod mat;
pub(self) mod mesh;
pub mod texture_manager;

pub use animation_manager::*;
pub use defns::*;
pub use macros::*;
pub use texture_manager::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationSet;

pub(super) struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<mat::AnimationMaterial>();
        app.register_type::<AnimationBodyData>();

        app.add_plugins(Material2dPlugin::<mat::AnimationMaterial>::default());
        app.add_plugins(defns::AnimationDefnsPlugin);
    }
}
