use gtk::prelude::*;
use gtk::{glib, Application};

mod ui;

const APP_ID: &str = "org.gtk_rs.wlshot";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}
