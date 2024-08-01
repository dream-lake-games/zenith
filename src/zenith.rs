use bevy::{prelude::*, window::WindowMode};

pub mod consts;
pub mod input;
pub mod layer;
pub mod macros;
pub mod roots;

pub mod prelude {
    pub use super::consts::*;
    pub use super::input::*;
    pub use super::layer::*;
    pub use super::macros::*;
    pub use super::roots::*;
    pub use bevy::prelude::*;
}

/// Registers all of the systems that are common to all platforms and then
/// runs the app.
pub fn launch_app(mut app: App) {
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
