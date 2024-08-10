pub use crate::prelude::*;

pub mod suicido;

pub use suicido::*;

impl Patrollable for Ship {}

pub(super) struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        register_patrol::<Ship>(app);

        suicido::register_suicidos(app);
    }
}
