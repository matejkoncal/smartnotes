use std::{
    env::var_os,
    fs,
    path::{Path, PathBuf},
    sync::mpsc::Receiver,
    time::{Duration, SystemTime},
};

use notify::Event;

pub fn debounce(
    rx: Receiver<std::result::Result<Event, notify::Error>>,
    handler: fn(path: &PathBuf),
) {
    let mut last_event_time = SystemTime::now();
    loop {
        let event = rx.recv().unwrap().unwrap();
        if let Ok(metadata) = fs::metadata(event.paths.first().unwrap()) {
            if event.kind.is_create() && metadata.is_file() {
                if SystemTime::now() > last_event_time.checked_add(Duration::from_secs(1)).unwrap()
                {
                    handler(event.paths.first().unwrap());
                }
                last_event_time = SystemTime::now();
            }
        }
    }
}

pub fn exist_dir(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_dir() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn get_output_dir() -> PathBuf {
    if let Some(smartnotes_out) = var_os("SMARTNOTES_OUT") {
        Path::new(&smartnotes_out).to_path_buf()
    } else {
        let home_var = var_os("HOME").unwrap();
        let home = Path::new(&home_var);
        home.join(Path::new("notes"))
    }
}
