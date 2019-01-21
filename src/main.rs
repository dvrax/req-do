extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;

use std::process::Command;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("interface.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Could not build interface");

    let main_window: gtk::ApplicationWindow = builder
        .get_object("main_window")
        .expect("Could not get main window");
    main_window.set_application(application);

    main_window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });

    let result_text: gtk::TextView = builder
        .get_object("result_text")
        .expect("Could not get output text view");

    let request_entry: gtk::Entry = builder
        .get_object("request_uri")
        .expect("Could not get entry");

    let submit_button: gtk::Button = builder
        .get_object("submit_request")
        .expect("Could not get submit button");

    submit_button.grab_default();
    submit_button.connect_clicked(move |_| {
        let entry_text = request_entry
            .get_text()
            .expect("Failed to get text from entry");
        if let Ok(curl_output) = Command::new("curl").arg("-L").arg(&entry_text).output() {
            if let Ok(curl_stdout) = std::str::from_utf8(&curl_output.stdout) {
                result_text
                    .get_buffer()
                    .expect("Couldn't get output text buffer!")
                    .set_text(curl_stdout);
            } else {
                result_text
                    .get_buffer()
                    .expect("Couldn't get output text buffer!")
                    .set_text("Failed to parse curl output");
            }
        } else {
            result_text
                .get_buffer()
                .expect("Couldn't get output text buffer!")
                .set_text("Failed to get requested uri");
        }
    });

    main_window.show_all();
}
fn main() {
    let application = gtk::Application::new("com.dvrax.req_do", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&std::env::args().collect::<Vec<_>>());
}
