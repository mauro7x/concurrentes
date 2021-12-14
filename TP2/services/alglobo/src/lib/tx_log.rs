use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
};

use crate::{
    constants::paths::LAST_PAYMENT_STATE,
    protocol::data::{cast_action_to_char, cast_char_to_action},
    types::{
        common::BoxResult,
        data::{Action, Tx},
    },
};

use log::*;

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
    pub fn get(&mut self, tx: &Tx) -> BoxResult<Option<Action>> {
        let res = self.in_memory_log.get(tx);

        match res {
            None => {
                if let Some((logged_tx, logged_action)) = self.read_log()? {
                    if logged_tx == *tx {
                        info!(
                            "State recovered from file! Congrats, it works \n
                        Transaction: {}, Action: {:?}",
                            logged_tx, logged_action
                        );
                        return Ok(Some(logged_action));
                    }
                };
                Ok(None)
            }
            Some(val) => Ok(Some(*val)),
        }
    }

    fn write_log(&mut self, tx: &Tx, action: Action) -> BoxResult<()> {
        self.file.rewind()?;
        let action_byte = &[cast_action_to_char(&action)];
        let action = std::str::from_utf8(action_byte)?;

        self.file
            .write_all(format!("{}{}\n", tx, action).as_bytes())?;

        // we ensure log is persisted to disk
        self.file.sync_all()?;
        Ok(())
    }

    fn read_log(&mut self) -> BoxResult<Option<(Tx, Action)>> {
        self.file.rewind()?;
        let reader = BufReader::new(&self.file);

        if let Some(line) = reader.lines().next() {
            let mut char_vec: Vec<char> = line?.chars().collect();

            if let Some(action) = char_vec.pop() {
                let s: String = char_vec.into_iter().collect();

                let id = s.parse::<u32>()?;

                return Ok(Some((id, cast_char_to_action(action as u8)?)));
            }
        }

        Ok(None)
    }
}
