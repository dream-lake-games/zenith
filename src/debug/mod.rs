use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
struct DebugState {}
impl Default for DebugState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Resource, Reflect)]
struct DebugInteractive(DebugState);
fn update_debug_state(
    interactive_state: Res<DebugInteractive>,
    debug_state: Res<State<DebugState>>,
    mut next_debug_state: ResMut<NextState<DebugState>>,
) {
    if &interactive_state.0 != debug_state.get() {
        next_debug_state.set(interactive_state.0.clone());
    }
}

fn set_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 4.0;
    config.render_layers = SpriteLayer::render_layers();
}

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)),
        );
        app.insert_state(DebugState::default());
        app.add_systems(Startup, set_gizmo_config);

        // Debug
        app.insert_resource(DebugInteractive(DebugState::default()));
        app.add_plugins(
            ResourceInspectorPlugin::<DebugInteractive>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
        app.add_systems(Update, update_debug_state.run_if(in_state(AppMode::Dev)));
    }
}
