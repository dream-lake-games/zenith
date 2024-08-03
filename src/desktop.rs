//! All of the resources, logic, etc. needed when running the desktop app

use bevy::window::WindowMode;
use zenith::prelude::*;

pub fn generate_desktop_app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    title: "ZENITH".into(),
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.add_plugins(DesktopPlugin);
    app
}

struct DesktopPlugin;
impl Plugin for DesktopPlugin {
    fn build(&self, app: &mut App) {}
}
