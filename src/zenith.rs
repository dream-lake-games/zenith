use bevy::{prelude::*, window::WindowMode};
use consts::FRAMERATE;

pub mod animation;
pub mod camera;
pub mod consts;
pub mod debug;
pub mod enemy;
pub mod environment;
pub mod input;
pub mod layer;
pub mod lifecycle;
pub mod macros;
pub mod math;
pub mod mirage;
pub mod particles;
pub mod physics;
pub mod roots;
pub mod ship;
pub mod state;

pub mod prelude {
    pub use super::animation::*;
    pub use super::camera::*;
    pub use super::consts::*;
    pub use super::debug::*;
    pub use super::enemy::*;
    pub use super::environment::*;
    pub use super::input::*;
    pub use super::layer::*;
    pub use super::lifecycle::*;
    pub use super::macros::*;
    pub use super::math::*;
    pub use super::mirage::*;
    pub use super::particles::*;
    pub use super::physics::*;
    pub use super::roots::*;
    pub use super::ship::*;
    pub use super::state::*;
    pub use bevy::color::palettes::tailwind;
    pub use bevy::input::common_conditions::input_toggle_active;
    pub use bevy::prelude::*;
    pub use bevy::render::view::*;
    pub use bevy::utils::{HashMap, HashSet};
    pub use bevy_inspector_egui::quick::ResourceInspectorPlugin;
    pub use rand::thread_rng;
    pub use rand::Rng;
    pub use std::collections::VecDeque;
    pub use std::time::Duration;
}

/// Registers all of the systems that are common to all platforms and then
/// runs the app.
pub fn launch_app(mut app: App) {
    app.insert_resource(Time::<Fixed>::from_hz(FRAMERATE as f64));
    app.add_plugins(layer::LayerPlugin::new(
        consts::IDEAL_VEC,
        consts::MENU_GROWTH,
    ));
    app.add_plugins(animation::AnimationPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(environment::EnvironmentPlugin);
    app.add_plugins(input::CommonInputPlugin);
    app.add_plugins(lifecycle::LifecyclePlugin);
    app.add_plugins(mirage::MiragePlugin);
    app.add_plugins(particles::ParticlesPlugin);
    app.add_plugins(physics::PhysicsPlugin);
    app.add_plugins(roots::RootPlugin);
    app.add_plugins(state::StatePlugin);
    app.run();
}

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                // on iOS, gestures must be enabled.
                // This doesn't work on Android
                recognize_rotation_gesture: true,
                ..default()
            }),
            ..default()
        }), // .set(ImagePlugin::default_nearest()),
    );
    launch_app(app);
}
