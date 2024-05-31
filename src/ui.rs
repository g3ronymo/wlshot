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
    FileDialog,
};
use gio::File;
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


    // ok button
    let hbox_buttons = Box::new(Orientation::Horizontal, 30);
    hbox_buttons.set_halign(Align::End);
    let ok_button = Button::builder()
        .label("Ok")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let window_ref = Rc::clone(&window);
    ok_button.connect_clicked(move |_| {
        let tmp_file = get_tmp_file();
        let default_dir = config.output_dir.clone();
        if region.is_active() {
            wlshot::capture(CaptureType::Region, tmp_file.clone());
        } else {
            wlshot::capture(CaptureType::Fullscreen, tmp_file.clone());
        }
        save_dialog(
            tmp_file.clone(),
            default_dir,
            default_filename(),
            Rc::clone(&window_ref)
        );
    });
    hbox_buttons.append(&ok_button);
    vbox.append(&hbox_buttons);
    window.set_child(Some(&vbox));
}


fn save_dialog(
    tmp_file: PathBuf, 
    initial_dir: PathBuf, 
    initial_file: String,
    window: Rc<ApplicationWindow>) {
    let dir = File::for_path(initial_dir);
    let file_dialog = FileDialog::builder()
        .initial_folder(&dir)
        .initial_name(initial_file)
        .build();
    file_dialog.save(
        None::<&ApplicationWindow>,
        None::<&gio::Cancellable>,
        move |c| {
            match c {
                Ok(file) => {
                    if let Some(p) = file.path() {
                        fs::copy(tmp_file, p)
                            .expect("Unable to copy temporary file");
                    }
                    window.close();
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            }
        }
    );
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

