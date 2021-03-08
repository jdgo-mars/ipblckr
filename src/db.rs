use crate::deny_cmd;
use pickledb::{error::Error, PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::path::Path;

const THRESHOLD: i32 = 2;
const BLOCKED_LIST: &'static str = "blocked_list";
const LINE_CURSOR: &'static str = "line_cursor";

pub struct Db {
    inner: PickleDb,
}

impl Db {
    fn ensure_blocked_list(&mut self) {
        if !self.inner.lexists(BLOCKED_LIST) {
            self.inner.lcreate(BLOCKED_LIST).unwrap();
        }
    }
    fn add_2_blocked_list(&mut self, ip: &str) {
        self.inner.ladd(BLOCKED_LIST, &String::from(ip));
        deny_cmd::add_2_iptables(ip);
    }

    fn is_blocked(&mut self, ip: &str) -> bool {
        for b in self.inner.liter(BLOCKED_LIST) {
            let c = b.get_item::<String>().unwrap();
            if ip == c {
                return true;
            }
        }

        return false;
    }

    pub fn dump_2_disk(&mut self) -> Result<(), Error> {
        return self.inner.dump();
    }

    pub fn set_line_cursor(&mut self, line_n: usize) {
        self.inner.set(LINE_CURSOR, &line_n).unwrap();
    }

    pub fn get_line_cursor(&self) -> usize {
        self.inner.get::<usize>(LINE_CURSOR).unwrap_or(0)
    }

    fn check_reached_limit(&mut self, ip: &str) {
        let count = self.inner.get::<i32>(ip).unwrap();
        if count == THRESHOLD {
            self.add_2_blocked_list(&ip);
        }
    }

    pub fn increment_count(&mut self, ip: &str) {
        self.ensure_blocked_list();
        self.check_reached_limit(ip);

        if !self.is_blocked(ip) {
            let count = self.inner.get::<i32>(ip).unwrap_or(0);
            let n = count + 1;
            self.inner.set(&ip, &n).unwrap();
        }
    }

    pub fn new() -> Db {
        let p_str = format!("{}/{}", dotenv!("DATA_PATH"), dotenv!("DB_NAME"));
        let db_path = Path::new(&p_str);
        let inner = PickleDb::load(
            db_path,
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        )
        .unwrap_or(PickleDb::new_json(
            db_path,
            PickleDbDumpPolicy::DumpUponRequest,
        ));

        Db { inner: inner }
    }
}
