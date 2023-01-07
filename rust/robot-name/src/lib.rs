#[macro_use]
extern crate lazy_static;
use rand::Rng;
use std::{collections::HashSet, sync::Mutex};

pub struct Robot {
    name: String,
}

lazy_static! {
    static ref EXISTING_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

impl Robot {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let letters: String = (0..2).map(|_| rng.gen_range('A'..='Z')).collect();
            let nums: String = (0..3).map(|_| rng.gen_range('0'..='9')).collect();
            let name = format!("{letters}{nums}");
            let mut existing_names = EXISTING_NAMES.lock().unwrap();
            if !existing_names.contains(&name) {
                existing_names.insert(name.clone());
                break Robot { name };
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        EXISTING_NAMES.lock().unwrap().remove(self.name());
        let new = Robot::new();
        self.name = new.name;
    }
}
