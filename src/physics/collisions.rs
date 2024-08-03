//! This is logic for reacting to collisions.
//! Perhaps confusingly, it runs _after_ the "collisions" set.
//! This is because "collisions" will create all the collisions, then this set
//! will do things like spawn sound effects, etc

use crate::prelude::*;

pub(super) fn register_collisions(_app: &mut App) {}
