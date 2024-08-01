use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, States)]
pub enum AppMode {
    Dev,
    Prod,
}
