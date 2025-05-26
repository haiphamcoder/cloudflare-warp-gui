mod shell;
mod status;
mod gui;
mod warp;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = Application::new(
        Some("com.haiphamcoder.cloudflare-warp-gui"),
        Default::default(),
    );

    app.connect_activate(|app| {
        gui::build_ui(app);
    });

    app.run();
}
