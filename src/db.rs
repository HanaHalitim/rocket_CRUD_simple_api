use std::sync::Mutex;
use crate::models::Record;

pub struct Db {
    pub records: Mutex<Vec<Record>>,
}

impl Db {
    pub fn new() -> Db {
        Db { records: Mutex::new(vec![]) }
    }
}
