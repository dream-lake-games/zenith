pub use crate::prelude::*;

pub mod suicido;

pub use suicido::*;

pub(super) struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {}
}
