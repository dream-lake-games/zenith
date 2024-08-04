use super::manager::*;
use crate::prelude::*;

pub mod ship_animation;

pub use ship_animation::*;

pub(super) struct AnimationDefnsPlugin;
impl Plugin for AnimationDefnsPlugin {
    fn build(&self, app: &mut App) {
        register_animation_manager::<AnimationShip>(app);
    }
}
