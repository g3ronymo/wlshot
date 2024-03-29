use std::{
    path::PathBuf,
    rc::Rc,
    fs,
    env, 
};

use gtk::prelude::*;
use gtk::{
    Application, 
    ApplicationWindow,
    Button,
    CheckButton,
    Box,
    Orientation,
    Align,
    glib::GString,
    Entry,
};
use chrono::{Local, SecondsFormat};

use wlshot::CaptureType;
use wlshot::Config;

pub fn build_ui(app: &Application) {
    let config = wlshot::Config::build();
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("wlshot")
        .build();
    let w_ref = Rc::new(window);
    let config_ref = Rc::new(config);
    screen_01(Rc::clone(&w_ref), Rc::clone(&config_ref));

    // Present window
    w_ref.present();
}

fn screen_01(window: Rc<ApplicationWindow>, config: Rc<Config>) {
    let vbox = Box::new(Orientation::Vertical, 30);

    // radio buttons to select capture type (fullscreen, screen region)
    let region = CheckButton::with_label("Region"); 
    let fullscreen = CheckButton::with_label("Fullscreen"); 
    match config.area {
        CaptureType::Region => region.set_active(true),
        CaptureType::Fullscreen => fullscreen.set_active(true),
    }

    let capture_type_group = CheckButton::new();
    region.set_group(Some(&capture_type_group));
    fullscreen.set_group(Some(&capture_type_group));

    let hbox_capture_types = Box::new(Orientation::Horizontal, 30);
    hbox_capture_types.append(&region);
    hbox_capture_types.append(&fullscreen);
    vbox.append(&hbox_capture_types);


    // ok and cancel button
    let hbox_buttons = Box::new(Orientation::Horizontal, 30);
    hbox_buttons.set_halign(Align::End);
    let ok_button = Button::builder()
        .label("Ok")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let wclone = Rc::clone(&window);
    ok_button.connect_clicked(move |_| {
        let tmp_file = get_tmp_file();
        if region.is_active() {
            wlshot::capture(CaptureType::Region, tmp_file);
            screen_02(Rc::clone(&wclone), Rc::clone(&config));
        } else {
            wlshot::capture(CaptureType::Fullscreen, tmp_file);
            screen_02(Rc::clone(&wclone), Rc::clone(&config));
        }
    });
    hbox_buttons.append(&ok_button);
    vbox.append(&hbox_buttons);
    window.set_child(Some(&vbox));
}


fn screen_02(window: Rc<ApplicationWindow>, config: Rc<Config>) {
    // display image
    let vbox = Box::new(Orientation::Vertical, 30);
    let tmp_file = get_tmp_file();
    let image = gtk::Picture::for_filename(tmp_file);
    vbox.append(&image);

    let mut default_dir = config.output_dir.clone();
    default_dir.push(default_filename());
    let file_entry = Entry::builder()
        .text(GString::from_string_unchecked(
                default_dir.to_str().unwrap().to_string()))
        .build();
    vbox.append(&file_entry);

    let hbox_buttons = Box::new(Orientation::Horizontal, 30);
    hbox_buttons.set_halign(Align::End);
    vbox.append(&hbox_buttons);

    let save_button = Button::builder()
        .label("Save")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    hbox_buttons.append(&save_button);
    let window_ref = Rc::clone(&window);
    save_button.connect_clicked(move |_| {
        let file_path = PathBuf::from(file_entry.text());
        if let Some(p) = file_path.parent() {
            if p.is_dir() {
                let tmp_file = get_tmp_file();
                fs::copy(tmp_file, file_path)
                    .expect("could not copy temporary file");
                window_ref.close();
            } else {
                // TODO warn
            }
        } else {
            // TODO warn
        }
    });
    window.set_child(Some(&vbox));
}

fn default_filename() -> String {
    let now = Local::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    format!("{}.png", now)
}

fn get_tmp_file() -> PathBuf {
    let mut path = env::temp_dir();
    let executable_name = env::args().next().unwrap();
    let executable_name = PathBuf::from(executable_name)
        .file_stem().unwrap().to_str().unwrap().to_string();
    path.push(executable_name);
    path
}

