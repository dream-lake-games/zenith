use crate::prelude::*;

pub mod desktop_input;

pub use desktop_input::*;

// Any place in the app that needs to react to input should use these events and resources
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

/// Drag input
#[derive(Resource)]
pub struct DragInput {
    screen_pos: Vec2,
    world_pos: Vec2,
    left_drag_start: Option<Vec2>,
    right_drag_start: Option<Vec2>,
}
impl DragInput {
    pub fn get_screen_pos(&self) -> Vec2 {
        self.screen_pos
    }

    pub fn get_world_pos(&self) -> Vec2 {
        self.world_pos
    }

    pub fn get_left_drag_start(&self) -> Option<Vec2> {
        self.left_drag_start
    }

    pub fn get_right_drag_start(&self) -> Option<Vec2> {
        self.right_drag_start
    }
}

/// Event that corresponds to input that _should_ send the ship flying.
/// NOTE: It is the responsibility of other systems to monitor the ship resources,
/// i.e. determine if it actually CAN go flying rn
#[derive(Event)]
pub struct Launch(pub Vec2);

/// Event that corresponds to input that _should_ fire a bullet.
/// NOTE: It is the responsibility of other systems to monitor the ship resources,
/// i.e. determine if it actually CAN go shooting rn
#[derive(Event)]
pub struct Fire(pub Vec2);

/// Input event controlling text boxes
#[derive(Event)]
pub struct ConvoGoNext;

/// Rn used to continue on some menu screens, maybe will expand to custom UI? idk
#[derive(Event)]
pub enum NonGameInput {
    Continue,
}

/// This plugin only defines the common input interfaces between platforms.
/// The code launching each platforms app should register one of the sub-mods
/// to actually get the logic that updates these input events and resources.
pub(super) struct CommonInputPlugin;
impl Plugin for CommonInputPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.insert_resource(DragInput {
            screen_pos: default(),
            world_pos: default(),
            left_drag_start: None,
            right_drag_start: None,
        });

        // Events
        app.add_event::<Launch>();
        app.add_event::<Fire>();
        app.add_event::<ConvoGoNext>();
        app.add_event::<NonGameInput>();
    }
}
