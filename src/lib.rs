pub mod assertions;
pub mod macros;

use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;

use crate::assertions::AssertionLevel;

/// Shared assertion storage value to work with
pub static STORE: OnceLock<Mutex<BasicStore>> = OnceLock::new();

/// Struct to store all assertions and access them via id
#[derive(Clone, Debug)]
pub struct BasicStore {
    /// All assertion with key - their id
    assertions: HashMap<u32, AssertionInfo>
}

pub fn basic_store() -> &'static Mutex<BasicStore> {
    STORE.get_or_init(|| {
        Mutex::new(BasicStore {
            assertions: HashMap::new(),
        })
    })
}

/// Struct to store assertion entry information
#[derive(Clone, Copy, Debug)]
pub struct AssertionInfo {
    /// Number of times assertion called
    times: u32,
    /// Assertion level, it sets during AssertionInfo creationg
    level: AssertionLevel,
}

impl BasicStore {
    /// Function to store assertion entries
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

    /// Function to print all assertion entries information
    pub fn print(&self) {
        for (id, info) in &self.assertions {
            println!("ID: {}, Level: {:?}, Times: {}", id, info.level, info.times);
        }
    }

    /// Resets all assertions information
    pub fn clear(&mut self) {
        self.assertions.clear();
    }

    /// Reterns clone version of all assertions
    pub fn dump(&self) -> HashMap<u32, AssertionInfo> {
        self.assertions.clone()
    }

    /// Return assertion entry information
    pub fn get(&self, asrt_id: u32) -> Option<AssertionInfo> {
        self.assertions.get(&asrt_id).copied()
    }
}

impl AssertionInfo {
    /// Return number of times assertion was called
    pub fn number_of_calls(&self) -> u32 {
        self.times
    }

    /// Returns level of assertion (uses first met assertion level with this id)
    pub fn level(&self) -> AssertionLevel {
        self.level
    }
}
