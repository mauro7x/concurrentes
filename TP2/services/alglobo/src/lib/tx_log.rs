use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Seek, Write},
};

use crate::{
    constants::paths::LAST_PAYMENT_STATE,
    protocol::data::cast_action,
    types::{
        common::BoxResult,
        data::{Action, Tx},
    },
};

pub struct TxLog {
    file: File,
    in_memory_log: HashMap<Tx, Action>,
}

impl TxLog {
    pub fn new() -> BoxResult<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(LAST_PAYMENT_STATE)?;

        Ok(TxLog {
            file,
            in_memory_log: HashMap::new(),
        })
    }
    pub fn insert(&mut self, tx: Tx, action: Action) -> BoxResult<()> {
        self.write_log(&tx, action)?;
        self.in_memory_log.insert(tx, action);
        Ok(())
    }
    pub fn get(&mut self, tx: &Tx) -> Option<&Action> {
        self.in_memory_log.get(tx)
    }

    fn write_log(&mut self, tx: &Tx, action: Action) -> std::io::Result<()> {
        self.file.rewind()?;

        self.file
            .write_all(format!("{}-{}", tx, cast_action(&action)).as_bytes())
    }
}
