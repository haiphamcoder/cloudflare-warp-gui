use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Switch, Box as GtkBox, Orientation};

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Cloudflare Warp Installer");
    window.set_default_size(500, 600);

    let vbox = GtkBox::new(Orientation::Vertical, 10);

    let status_label = Label::new(Some("Checking status..."));
    let switch = Switch::new();
    let install_button = Button::with_label("Install Warp");
    let refresh_button = Button::with_label("Refresh status");

    vbox.pack_start(&status_label, false, false, 10);
    vbox.pack_start(&switch, false, false, 10);
    vbox.pack_start(&install_button, false, false, 10);
    vbox.pack_start(&refresh_button, false, false, 10);

    window.add(&vbox);
    window.show_all();
}