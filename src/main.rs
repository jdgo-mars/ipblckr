#[macro_use]
extern crate dotenv_codegen;
mod db;
mod deny_cmd;
mod matcher;

use db::Db;
use dotenv::dotenv;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::{create_dir_all, File};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::{sync::mpsc::channel, time::Duration};

const DATA_PATH: &str = dotenv!("DATA_PATH");
const LOGS_PATH: &str = dotenv!("LOGS_PATH");

fn ensure_path(p: &str) {
    let p = Path::new(p);
    if !p.exists() {
        create_dir_all(p).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    ensure_path(DATA_PATH);
    ensure_path(LOGS_PATH);

    let mut db = Db::new();

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

    watcher
        .watch(dotenv!(WATCH_PATH), RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match &event {
                DebouncedEvent::Write(path) => {
                    let path = String::from(path.to_str().unwrap());
                    if path.contains("auth.log") {
                        let file = File::open(path)?;
                        let reader = BufReader::new(file);
                        let cursor = db.get_line_cursor();
                        for (n, line) in reader.lines().enumerate().skip(cursor) {
                            let line = line.unwrap().into_bytes();
                            let ip = matcher::get_ip(&line);

                            match &ip {
                                Some(ip) => {
                                    db.set_line_cursor(n);
                                    db.increment_count(ip);
                                }
                                None => (),
                            }
                        }
                        db.dump_2_disk().unwrap();
                    }
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        };
    }
}
