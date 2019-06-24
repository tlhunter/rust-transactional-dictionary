/**
 * This is basically a wrapper around HashMap which enables transactions
 * Once a transaction has been started any changes won't be permanent
 * The transaction can then be committed or canceled
 */

use std::collections::HashMap;
use std::collections::HashSet;
enum Mode {
    Normal,
    Transaction,
}

// TODO: Make generic instead of <String, String>

pub struct Dict {
    data: HashMap<String, String>,
    transaction_data: HashMap<String, String>,
    transaction_deletes: HashSet<String>, // deletes are separate for merge purposes
    mode: Mode,
}

impl Dict {
    pub fn new() -> Dict {
        Dict {
            data: HashMap::new(),
            transaction_data: HashMap::new(),
            transaction_deletes: HashSet::new(),
            mode: Mode::Normal,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        match self.mode {
            Mode::Transaction => {
                self.transaction_deletes.remove(&key);
                self.transaction_data.insert(key, value);
            }
            Mode::Normal => {
                self.data.insert(key, value);
            }
        };
    }

    pub fn get(&self, key: String) -> Option<&String> {
        if let Mode::Transaction = self.mode {
            if let Some(value) = self.transaction_data.get(&key) {
                return Some(value);
            }

            if self.transaction_deletes.contains(&key) {
                return None;
            }
        }

        self.data.get(&key)
    }

    pub fn delete(&mut self, key: String) {
        match self.mode {
            Mode::Transaction => {
                self.transaction_data.remove(&key);
                self.transaction_deletes.insert(key);
            }
            Mode::Normal => {
                self.data.remove(&key);
            }
        }
    }

    pub fn has(&self, key: String) -> bool {
        match self.get(key) {
            Some(_i) => true,
            None => false,
        }
    }

    // begins a transaction
    pub fn begin(&mut self) -> bool {
        match self.mode {
            Mode::Transaction => false,
            Mode::Normal => {
                self.mode = Mode::Transaction;
                true
            }
        }
    }

    // commits transaction
    pub fn commit(&mut self) -> bool {
        match self.mode {
            Mode::Normal => false,
            Mode::Transaction => {
                for (key, value) in self.transaction_data.iter() {
                    self.data.insert(key.to_string(), value.to_string());
                }
                for key in self.transaction_deletes.iter() {
                    self.data.remove(&key.to_string());
                }
                self.mode = Mode::Normal;
                self.transaction_data.clear();
                self.transaction_deletes.clear();
                true
            }
        }
    }

    // cancels transaction
    pub fn cancel(&mut self) -> bool {
        match self.mode {
            Mode::Normal => false,
            Mode::Transaction => {
                self.mode = Mode::Normal;
                self.transaction_data.clear();
                self.transaction_deletes.clear();
                true
            }
        }
    }

    // TODO: need a better way to do this...
    pub fn inspect(&self) {
        println!("\ncommitted: {:#?}", self.data);
        println!("uncommitted deletes: {:#?}\n", self.transaction_deletes);
        println!("uncommitted: {:#?}\n", self.transaction_data);
    }
}
