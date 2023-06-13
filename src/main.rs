use daemonize::Daemonize;
use notify::{RecursiveMode, Watcher};
use std::fs::{self, File};
use std::{env::current_dir, path::PathBuf, sync::mpsc::channel};
use tools::{debounce, exist_dir, get_new_file_path};

mod ai;
mod tools;

fn main() {
    let stdout = File::create("/tmp/smartnotes.out").unwrap();
    let stderr = File::create("/tmp/smartnotes.err").unwrap();

    let argument = std::env::args().nth(1).expect("No argument provided");
    let watched_dir = current_dir().unwrap().join(PathBuf::from(argument));

    if exist_dir(&watched_dir) {
        Daemonize::new()
            .stderr(stderr)
            .stdout(stdout)
            .start()
            .unwrap();

        tokio_main(&watched_dir);
    } else {
        println!("Folder does not exist.");
    }
}

#[tokio::main]
async fn tokio_main(watched_dir: &PathBuf) {
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx).unwrap();

    watcher
        .watch(watched_dir, RecursiveMode::Recursive)
        .unwrap();

    debounce(rx, |path| {
        let original_file = fs::read_to_string(&path).unwrap();
        let new_path = get_new_file_path(&path);

        tokio::spawn(async move {
            let transformed_file = ai::transform_to_md(&original_file).await;
            fs::write(new_path, transformed_file).unwrap();
        });
    });
}
