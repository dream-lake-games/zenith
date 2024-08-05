use super::animation_manager::*;
use super::texture_manager::*;
use crate::prelude::*;

pub mod planet_textures;
pub mod ship_animation;

pub use planet_textures::*;
pub use ship_animation::*;

pub(super) struct AnimationDefnsPlugin;
impl Plugin for AnimationDefnsPlugin {
    fn build(&self, app: &mut App) {
        register_animation_manager::<AnimationShip>(app);

        register_texture_manager::<TextureNormalPlanetState>(app);
        register_texture_manager::<TextureTestPlanetState>(app);
    }
}
