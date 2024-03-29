use std::{
    process::Command, 
    path::PathBuf, 
    fs,
};

pub enum CaptureType {
    Region,
    Fullscreen,
}

pub struct Config {
    /// default CaptureType
    pub area: CaptureType,
    /// default output directory
    pub output_dir: PathBuf,
}

impl Config {
    pub fn build() -> Self {
        let (_, home) = std::env::vars().find(|(k,_)| k == "HOME")
            .expect("$HOME not set");
        let mut config = Config {
            area: CaptureType::Region,
            output_dir: PathBuf::from(home),
        };

        // find config and process it 
        let arg1 = std::env::args().next().unwrap();
        let progname = PathBuf::from(arg1);
        let progname = progname.file_name()
            .expect("Unable to get executable name");
        let base_directories = xdg::BaseDirectories::with_prefix(&progname)
            .expect("Unable to find base directories");

        let mut config_file = progname.to_os_string();
        config_file.push(".toml");
        let config_file = base_directories.find_config_file(&config_file);
        if let Some(conf) = config_file {
            let conf = fs::read_to_string(conf)
                .expect("unable to read config");
            for line in conf.lines() {
                if let Some((k,v)) = line.split_once('=') {
                    if k == "default_dir" {
                        let output_dir = PathBuf::from(v);
                        if !output_dir.is_dir() {
                            let msg: String = format!(
                                "{}: {} -- is not a directory", 
                                conf, line);
                            eprintln!("{}", msg);
                        } else {
                            config.output_dir = output_dir;
                        }
                    }
                }
            }

        }
        config
    }
}

pub fn capture(capture_type: CaptureType, output_file: PathBuf) {
    match capture_type {
        CaptureType::Region => capture_region(output_file),
        CaptureType::Fullscreen => capture_fullscreen(output_file),

    }
}

fn capture_region(output_file: PathBuf) {
    let area = Command::new("slurp")
        .output()
        .expect("failed to execute slurp");
    let area = String::from_utf8(area.stdout).unwrap();
    Command::new("grim")
        .arg("-g")
        .arg(area.as_str().trim())
        .arg(&output_file)
        .status()
        .expect("failed to execute grim");
}

fn capture_fullscreen(output_file: PathBuf) {
    Command::new("grim")
        .arg(&output_file)
        .status()
        .expect("failed to execute grim");
}
