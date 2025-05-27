use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box as GtkBox, Orientation, MenuButton, Popover, Image, IconSize};

use crate::warp::{check_warp_status, warp_connect, warp_disconnect, warp_register};
use crate::status::WarpStatus;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Cloudflare Warp");
    window.set_default_size(500, 600);

    let vbox = GtkBox::new(Orientation::Vertical, 10);

    let status_label = Label::new(Some("Checking status..."));

    // Footer
    let footer = GtkBox::new(Orientation::Horizontal, 10);
    let logo_label = Label::new(Some("Cloudflare Warp"));
    let beta_label = Label::new(Some("Beta"));

    // Menu icon
    let menu_button = MenuButton::new();
    let menu_icon = Image::from_icon_name(Some("open-menu-symbolic"), IconSize::Menu);
    menu_button.set_child(Some(&menu_icon));
    menu_button.set_tooltip_text(Some("Menu"));

    // Create popover and menu box
    let popover = Popover::new(Some(&menu_button));
    let menu_box = GtkBox::new(Orientation::Vertical, 0);
    
    // Add menu items
    let connect_button = Button::with_label("Connect");
    let disconnect_button = Button::with_label("Disconnect");
    let register_button = Button::with_label("Register");
    let preferences_button = Button::with_label("Preferences");
    let about_button = Button::with_label("About");

    // Add click handlers
    let connect_button_clone = connect_button.clone();
    connect_button.connect_clicked(move |_| {
        if let Err(e) = warp_connect() {
            println!("Failed to connect: {}", e);
        }
    });

    let disconnect_button_clone = disconnect_button.clone();
    disconnect_button.connect_clicked(move |_| {
        if let Err(e) = warp_disconnect() {
            println!("Failed to disconnect: {}", e);
        }
    });

    let register_button_clone = register_button.clone();
    register_button.connect_clicked(move |_| {
        if let Err(e) = warp_register() {
            println!("Failed to register: {}", e);
        }
    });

    menu_box.pack_start(&connect_button, false, false, 0);
    menu_box.pack_start(&disconnect_button, false, false, 0);
    menu_box.pack_start(&register_button, false, false, 0);
    menu_box.pack_start(&preferences_button, false, false, 0);
    menu_box.pack_start(&about_button, false, false, 0);

    // Add menu box to popover
    popover.set_child(Some(&menu_box));
    menu_button.set_popover(Some(&popover));

    footer.pack_start(&logo_label, false, false, 10);
    footer.pack_end(&menu_button, false, false, 10);
    footer.pack_end(&beta_label, false, false, 10);

    vbox.pack_start(&status_label, false, false, 10);
    vbox.pack_end(&footer, false, false, 10);

    window.add(&vbox);
    window.show_all();

    // Update status_label realtime every 1 seconds
    let status_label_clone = status_label.clone();
    glib::timeout_add_seconds_local(1, move || {
        let status = check_warp_status();
        
        let status_text = match status {
            WarpStatus::NotInstalled => "Warp is not installed",
            WarpStatus::Disconnected => "Warp is disconnected",
            WarpStatus::Connected => "Warp is connected",
            WarpStatus::Installing => "Installing Warp...",
            WarpStatus::Registering => "Registering Warp...",
            WarpStatus::Connecting => "Connecting to Warp...",
            WarpStatus::Error(msg) => &format!("Error: {}", msg),
        };
        
        status_label_clone.set_text(status_text);
        glib::Continue(true)
    });
}