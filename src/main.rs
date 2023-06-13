use daemonize::Daemonize;
use notify::{RecursiveMode, Watcher};
use std::fs::{self, File};
use std::path::Path;
use std::{env::current_dir, path::PathBuf, sync::mpsc::channel};
use tools::{debounce, exist_dir, get_output_dir};

mod ai;
mod tools;

fn main() {
    let stdout = File::create("/tmp/smartnotes.out").unwrap();
    let stderr = File::create("/tmp/smartnotes.err").unwrap();
    Daemonize::new()
        .stderr(stderr)
        .stdout(stdout)
        .start()
        .unwrap();

    tokio_main()
}

#[tokio::main]
async fn tokio_main() {
    let argument = std::env::args().nth(1).expect("No argument provided");
    let watched_dir = current_dir().unwrap().join(PathBuf::from(argument));

    if exist_dir(&watched_dir) {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx).unwrap();

        watcher
            .watch(&watched_dir, RecursiveMode::Recursive)
            .unwrap();

        debounce(rx, |path| {
            let path_copy = path.clone();
            println!("hovno");
            tokio::spawn(async move {
                println!("hovno1");
                let original_file = fs::read_to_string(&path_copy).unwrap();
                let transformed_file = ai::transform_to_md(&original_file).await;
                let output_dir = get_output_dir();

                if !exist_dir(&output_dir) {
                    fs::create_dir_all(&output_dir).unwrap();
                }

                let new_path = get_output_dir().join(Path::new(path_copy.file_name().unwrap()));
                fs::write(new_path, transformed_file).unwrap();
            });
        });
    }

    println!("Folder does not exist.");
}
