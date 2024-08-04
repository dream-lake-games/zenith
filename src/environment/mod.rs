use crate::prelude::*;

pub mod planet;

pub use planet::*;

pub(super) struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, _app: &mut App) {}
}
