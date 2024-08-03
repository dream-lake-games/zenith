use bevy::sprite::Material2dPlugin;
use manager::BodyData;

use crate::prelude::*;

mod defns;
pub mod macros;
pub mod manager;
pub(self) mod mat;

pub use macros::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationSet;

pub(super) struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<mat::AnimationMaterial>();
        app.register_type::<BodyData>();

        app.add_plugins(Material2dPlugin::<mat::AnimationMaterial>::default());
        app.add_plugins(defns::AnimationDefnsPlugin);
    }
}
