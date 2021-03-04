#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::{env, fs, path::Path};

use pickledb::{PickleDb, PickleDbDumpPolicy};

const DATA_PATH: &str = dotenv!("DATA_PATH");
const LOGS_PATH: &str = dotenv!("LOGS_PATH");

fn ensure_path(p: &str) {
    let p = Path::new(p);
    if !p.exists() {
        fs::create_dir_all(p).unwrap();
    }
}

fn main() {
    ensure_path(DATA_PATH);
    ensure_path(LOGS_PATH);

    let mut db = PickleDb::new_json(dotenv!("DB_NAME"), PickleDbDumpPolicy::AutoDump);
}
