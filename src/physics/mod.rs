use crate::prelude::*;

pub mod bounds;
pub mod bullet_time;
pub mod collisions;
pub mod dyno;
mod logic;
pub mod statics;
pub mod triggers;

pub use bounds::*;
pub use bullet_time::*;
// pub use collisions::*;
pub use dyno::*;
pub use statics::*;
pub use triggers::*;

/// The set that contains all physics related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

/// An internal set within physics that resolves all collisions
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CollisionsSet;

/// When a bunch of physics objects are first spawned in with same root,
/// I think it sometimes takes a tick for their transforms to stop interrupting
/// This struct basically marks any object whose physics stuff is "ready" (been around >= 1 frame)
#[derive(Component)]
struct PhysicsInitialized;

pub(super) struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bullet_time::BulletTimePlugin);
        collisions::register_collisions(app);
        logic::register_logic(app);
    }
}
