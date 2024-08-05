use crate::prelude::*;

pub mod mirage_drawing;

pub use mirage_drawing::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MirageSet;

pub(super) struct MiragePlugin;
impl Plugin for MiragePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MirageAnimationManager>();
        app.register_type::<MirageTextureManager>();

        mirage_drawing::register_mirage_drawing(app);
    }
}
