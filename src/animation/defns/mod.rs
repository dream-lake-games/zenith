use super::animation_manager::*;
use super::texture_manager::*;
use crate::prelude::*;

pub mod juice;
pub mod planet_textures;
pub mod ship_animation;
pub mod star_animation;
pub mod suicido_animation;

pub use juice::*;
pub use planet_textures::*;
pub use ship_animation::*;
pub use star_animation::*;
pub use suicido_animation::*;

pub(super) struct AnimationDefnsPlugin;
impl Plugin for AnimationDefnsPlugin {
    fn build(&self, app: &mut App) {
        register_animation_manager::<AnimationRingShrink>(app);
        register_animation_manager::<AnimationShipBody>(app);
        register_animation_manager::<AnimationShipCannon>(app);
        register_animation_manager::<AnimationShipTail>(app);
        register_animation_manager::<AnimationShipBulletDefault>(app);
        register_animation_manager::<AnimationStar>(app);
        register_animation_manager::<AnimationSuicidoBody>(app);
        register_animation_manager::<AnimationSuicidoExplosionCircle>(app);

        register_texture_manager::<TextureNormalPlanetState>(app);
        register_texture_manager::<TextureTestPlanetState>(app);
    }
}
