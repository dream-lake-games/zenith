use crate::prelude::*;

pub mod planet;
pub mod star;

pub use planet::*;
pub use star::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(star::StarPlugin);
    }
}
