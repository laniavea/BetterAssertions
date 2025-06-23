pub mod assertions;
pub mod macros;

use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

use crate::assertions::AssertionLevel;

pub static STORE: OnceLock<Mutex<BasicStore>> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct BasicStore {
    assertions: HashMap<u32, AssertionInfo>
}

pub fn basic_store() -> &'static Mutex<BasicStore> {
    STORE.get_or_init(|| {
        Mutex::new(BasicStore {
            assertions: HashMap::new(),
        })
    })
}

#[derive(Clone, Copy, Debug)]
pub struct AssertionInfo {
    times: u32,
    level: AssertionLevel,
}

impl BasicStore {
    pub fn store(&mut self, assertion_id: u32, assertion_lvl: AssertionLevel) {
        match self.assertions.get_mut(&assertion_id) {
            Some(a) => {
                a.times += 1
            },
            None => {
                self.assertions.insert(assertion_id, AssertionInfo {
                    times: 1,
                    level: assertion_lvl,
                });
            }
        }
    }

    pub fn print(&self) {
        for (id, info) in &self.assertions {
            println!("ID: {}, Level: {:?}, Times: {}", id, info.level, info.times);
        }
    }

    pub fn clear(&mut self) {
        self.assertions.clear();
    }

    pub fn dump(&self) -> HashMap<u32, AssertionInfo> {
        self.assertions.clone()
    }
}

impl AssertionInfo {
    pub fn number_of_calls(&self) -> u32 {
        self.times
    }

    pub fn level(&self) -> AssertionLevel {
        self.level
    }
}
