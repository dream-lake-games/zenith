use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum EncounterKind {
    SimpOnly,
    SpewOnly,
    Both,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, Default)]
pub enum EncounterProgress {
    #[default]
    Entering,
    Fighting,
    Meandering,
    Dead,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub struct EncounterState {
    pub kind: EncounterKind,
    pub difficulty: u32,
    pub progress: EncounterProgress,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub struct RoomState {
    pub room_size: UVec2,
    pub encounter_state: EncounterState,
}
impl RoomState {
    pub fn xth_encounter(kind: EncounterKind, difficulty: u32) -> Self {
        Self {
            room_size: IDEAL_VEC * 2,
            encounter_state: EncounterState {
                kind,
                difficulty,
                progress: EncounterProgress::Entering,
            },
        }
    }

    /// The next room to go to (assuming the bird doesn't die, or if it is dead, wants to play again)
    pub fn next_room(&self) -> Self {
        match (self.encounter_state.kind, self.encounter_state.difficulty) {
            (EncounterKind::SimpOnly, d) => {
                if d < 3 {
                    RoomState {
                        room_size: self.room_size,
                        encounter_state: EncounterState {
                            kind: EncounterKind::SimpOnly,
                            difficulty: d + 1,
                            progress: EncounterProgress::Entering,
                        },
                    }
                } else {
                    RoomState {
                        room_size: self.room_size,
                        encounter_state: EncounterState {
                            kind: EncounterKind::SpewOnly,
                            difficulty: 1,
                            progress: EncounterProgress::Entering,
                        },
                    }
                }
            }
            (EncounterKind::SpewOnly, _) => RoomState {
                room_size: self.room_size,
                encounter_state: EncounterState {
                    kind: EncounterKind::Both,
                    difficulty: 1,
                    progress: EncounterProgress::Entering,
                },
            },
            (EncounterKind::Both, d) => RoomState {
                room_size: self.room_size,
                encounter_state: EncounterState {
                    kind: EncounterKind::Both,
                    difficulty: d + 1,
                    progress: EncounterProgress::Entering,
                },
            },
        }
    }
}

impl ComputedStates for EncounterKind {
    type SourceStates = MetaState;

    fn compute(sources: MetaState) -> Option<Self> {
        match sources.get_room_state() {
            Some(room_state) => Some(room_state.encounter_state.kind),
            None => None,
        }
    }
}

impl ComputedStates for EncounterProgress {
    type SourceStates = MetaState;

    fn compute(sources: MetaState) -> Option<Self> {
        match sources.get_room_state() {
            Some(room_state) => Some(room_state.encounter_state.progress),
            None => None,
        }
    }
}

impl ComputedStates for EncounterState {
    type SourceStates = MetaState;

    fn compute(sources: MetaState) -> Option<Self> {
        match sources.get_room_state() {
            Some(room_state) => Some(room_state.encounter_state),
            None => None,
        }
    }
}

impl ComputedStates for RoomState {
    type SourceStates = MetaState;

    fn compute(sources: MetaState) -> Option<Self> {
        sources.get_room_state()
    }
}

pub(super) fn register_room_states(app: &mut App) {
    app.add_computed_state::<EncounterKind>();
    app.add_computed_state::<EncounterProgress>();
    app.add_computed_state::<EncounterState>();
    app.add_computed_state::<RoomState>();
}
