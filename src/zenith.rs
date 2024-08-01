use bevy::{prelude::*, window::WindowMode};

pub mod consts;
pub mod debug;
pub mod input;
pub mod layer;
pub mod macros;
pub mod roots;
pub mod state;

pub mod prelude {
    pub use super::consts::*;
    pub use super::debug::*;
    pub use super::input::*;
    pub use super::layer::*;
    pub use super::macros::*;
    pub use super::roots::*;
    pub use super::state::*;
    pub use bevy::input::common_conditions::input_toggle_active;
    pub use bevy::prelude::*;
    pub use bevy_inspector_egui::quick::ResourceInspectorPlugin;
}

/// Registers all of the systems that are common to all platforms and then
/// runs the app.
pub fn launch_app(mut app: App) {
    app.add_plugins(debug::DebugPlugin);
    app.add_plugins(layer::LayerPlugin::new(
        consts::IDEAL_VEC,
        consts::MENU_GROWTH,
    ));
    app.add_plugins(roots::RootPlugin);
    app.run();
}

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            // on iOS, gestures must be enabled.
            // This doesn't work on Android
            recognize_rotation_gesture: true,
            ..default()
        }),
        ..default()
    }));
    launch_app(app);
}
