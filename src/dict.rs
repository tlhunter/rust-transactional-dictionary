/**
 * This is basically a wrapper around HashMap which enables transactions
 * Once a transaction has been started any changes won't be permanent
 * The transaction can then be committed or canceled
 */
use std::collections::HashMap;

enum Mode {
    Normal,
    Transaction,
}

// TODO: Make generic instead of <String, String>

pub struct Dict {
    pub data: HashMap<String, String>,
    transaction_data: HashMap<String, String>,
    transaction_deletes: Vec<String>, // deletes are separate for merge purposes
    mode: Mode,
}

impl Dict {
    pub fn new() -> Dict{
        Dict {
            data: HashMap::new(),
            transaction_data: HashMap::new(),
            transaction_deletes: Vec::new(),
            mode: Mode::Normal,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        // TODO: if in transaction
            // add to transaction_data
            // remove from transaction_deletes
        self.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        // TODO: if in transaction
            // get from transaction_data
            // if not present fallback to data
        self.data.get(&key)
    }

    pub fn delete(&mut self, key: String) {
        // TODO: if in transaction
            // remove from transaction_data
            // add to transaction_deletes
        self.data.remove(&key);
    }

    pub fn has(&self, key: String) -> bool {
        // TODO: if in transaction
            // return false if in transaction_deletes
            // if in transaction_data return true
            // if not return if in data
        match self.data.get(&key) {
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
                // TODO: copy entries from transaction_data to data
                // TODO: delete entries in transaction_deletes from data
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
}
