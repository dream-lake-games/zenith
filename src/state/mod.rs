use crate::prelude::*;

pub mod room;

pub use room::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, States)]
pub enum AppMode {
    Dev,
    Prod,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum MenuState {
    Studio,
    Title,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum CutsceneState {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum TutorialState {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum MetaState {
    Menu(MenuState),
    Cutscene(CutsceneState),
    Tutorial(TutorialState),
    Room(RoomState),
    Transition,
}

/// Kills some verbosity with reading meta states
pub trait MetaUnfucker {
    fn get_menu_state(&self) -> Option<MenuState>;
    fn get_cutscene_state(&self) -> Option<CutsceneState>;
    fn get_tutorial_state(&self) -> Option<TutorialState>;
    fn get_room_state(&self) -> Option<RoomState>;
}
impl MetaUnfucker for MetaState {
    fn get_menu_state(&self) -> Option<MenuState> {
        match self {
            MetaState::Menu(menu_state) => Some(menu_state.clone()),
            _ => None,
        }
    }

    fn get_cutscene_state(&self) -> Option<CutsceneState> {
        match self {
            MetaState::Cutscene(cutscene_state) => Some(cutscene_state.clone()),
            _ => None,
        }
    }

    fn get_tutorial_state(&self) -> Option<TutorialState> {
        match self {
            MetaState::Tutorial(tutorial_state) => Some(tutorial_state.clone()),
            _ => None,
        }
    }

    fn get_room_state(&self) -> Option<RoomState> {
        match self {
            MetaState::Room(room_state) => Some(room_state.clone()),
            _ => None,
        }
    }
}
impl MetaUnfucker for State<MetaState> {
    fn get_menu_state(&self) -> Option<MenuState> {
        MetaState::get_menu_state(self.get())
    }

    fn get_cutscene_state(&self) -> Option<CutsceneState> {
        MetaState::get_cutscene_state(self.get())
    }

    fn get_tutorial_state(&self) -> Option<TutorialState> {
        MetaState::get_tutorial_state(self.get())
    }

    fn get_room_state(&self) -> Option<RoomState> {
        MetaState::get_room_state(self.get())
    }
}

/// Kills some verbosity for writing meta states
pub trait ToMetaState {
    fn to_meta_state(&self) -> MetaState;
}
macro_rules! impl_to_meta_state {
    ($type:ty, $disc:ident) => {
        impl ToMetaState for $type {
            fn to_meta_state(&self) -> MetaState {
                MetaState::$disc(self.clone())
            }
        }
    };
}
impl_to_meta_state!(MenuState, Menu);
impl_to_meta_state!(CutsceneState, Cutscene);
impl_to_meta_state!(TutorialState, Tutorial);
impl_to_meta_state!(RoomState, Room);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum PauseState {
    Unpaused,
    Paused,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum PhysicsState {
    Inactive,
    Active,
}

impl ComputedStates for PhysicsState {
    type SourceStates = (MetaState, PauseState);

    fn compute(sources: (MetaState, PauseState)) -> Option<Self> {
        // Here we convert from our [`AppState`] to all potential [`IsPaused`] versions.
        match sources {
            (MetaState::Tutorial(_), PauseState::Unpaused) => Some(Self::Active),
            (MetaState::Room(_), PauseState::Unpaused) => Some(Self::Active),
            _ => Some(Self::Inactive),
        }
    }
}

/// The state that actually holds data about transitions
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum MetaTransitionState {
    Stable,
    Leaving { next_meta_state: MetaState },
    Waiting { next_meta_state: MetaState },
    Entering,
}

/// Basically just std::mem::discriminant of the MetaTransitionState
/// Useful for getting access to OnEnter, OnExit transition stuff
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum TransitionState {
    Stable,
    Leaving,
    Waiting,
    Entering,
}

impl ComputedStates for TransitionState {
    type SourceStates = MetaTransitionState;

    fn compute(sources: MetaTransitionState) -> Option<Self> {
        // Here we convert from our [`AppState`] to all potential [`IsPaused`] versions.
        match sources {
            MetaTransitionState::Stable => Some(TransitionState::Stable),
            MetaTransitionState::Leaving { .. } => Some(TransitionState::Leaving),
            MetaTransitionState::Waiting { .. } => Some(TransitionState::Waiting),
            MetaTransitionState::Entering { .. } => Some(TransitionState::Entering),
        }
    }
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        // Ground truth states
        app.insert_state(AppMode::Dev);
        app.insert_state(MetaState::Room(RoomState::xth_encounter(
            EncounterKind::SimpOnly,
            1,
        ))); // INITIAL STATE (control f this silly)
        app.insert_state(MetaTransitionState::Stable);
        app.insert_state(PauseState::Unpaused);
        // Computed states
        app.add_computed_state::<PhysicsState>();
        app.add_computed_state::<TransitionState>();
        // Overcrowded states
        room::register_room_states(app);
    }
}
