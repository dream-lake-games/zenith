use bevy::window::PrimaryWindow;

use crate::prelude::*;

/// Updates the `DragInput` resource.
fn update_drag_input(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<DragInput>,
    mut launch_writer: EventWriter<Launch>,
    mut fire_writer: EventWriter<Fire>,
    ideal_mult: Res<IdealMult>,
    mut force_launches: EventReader<ForceLaunch>,
    mut force_fires: EventReader<ForceFire>,
) {
    let window = q_windows.single();
    let Some(mouse_pos) = window.cursor_position() else {
        // Mouse is not in the window, don't do anything
        return;
    };
    let should_force_launch = force_launches.read().last().is_some();
    let should_force_fire = force_fires.read().last().is_some();
    // TODO: Once we have a camera, screen/world pos calc needs to change
    let screen_pos = Vec2::new(
        mouse_pos.x - IDEAL_WIDTH_f32 * ideal_mult.0 / 2.0,
        -mouse_pos.y + IDEAL_HEIGHT_f32 * ideal_mult.0 / 2.0,
    ) / ideal_mult.0;
    let world_pos = screen_pos;
    let left_drag_start = if buttons.just_pressed(MouseButton::Left) {
        Some(screen_pos)
    } else {
        if let Some(drag_start) = state.left_drag_start {
            if !buttons.pressed(MouseButton::Left)
                || buttons.just_released(MouseButton::Left)
                || should_force_launch
            {
                launch_writer.send(Launch(drag_start - screen_pos));
                None
            } else {
                Some(drag_start)
            }
        } else {
            None
        }
    };
    let right_drag_start = if buttons.just_pressed(MouseButton::Right) {
        Some(screen_pos)
    } else {
        if let Some(drag_start) = state.right_drag_start {
            if !buttons.pressed(MouseButton::Right)
                || buttons.just_released(MouseButton::Right)
                || should_force_fire
            {
                fire_writer.send(Fire(drag_start - screen_pos));
                None
            } else {
                Some(drag_start)
            }
        } else {
            None
        }
    };
    *state = DragInput {
        screen_pos,
        world_pos,
        left_drag_start,
        right_drag_start,
    };
}

fn update_convo_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut convo: EventWriter<ConvoGoNext>,
) {
    if keyboard.any_just_pressed([
        KeyCode::Space,
        KeyCode::KeyA,
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyD,
    ]) || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right])
    {
        convo.send(ConvoGoNext);
    }
}

/// Send any and all non-game input. Note the early returns, we only handle at most one
/// such input per frame
fn update_non_game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut non_game_writer: EventWriter<NonGameInput>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        non_game_writer.send(NonGameInput::Continue);
        return;
    }
}

pub struct DesktopInputPlugin;
impl Plugin for DesktopInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_drag_input, update_convo_input, update_non_game_input),
        );
    }
}
