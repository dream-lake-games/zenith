use desktop::generate_desktop_app;
use zenith::launch_app;

mod desktop;

fn main() {
    let app = generate_desktop_app();
    launch_app(app);
}
