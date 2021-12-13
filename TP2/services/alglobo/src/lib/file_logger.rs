use crate::types::data::{Action, Tx};
use std::{fs::File, io::Write};

pub struct FileLogger {
    file: File,
}

impl FileLogger {
    pub fn new(filename: &str) -> Self {
        FileLogger {
            file: File::create(filename).expect("FileLog.new: error opening/creating file"),
        }
    }

    pub fn log(&mut self, tx: Tx, action: &Action) {
        self.file
            .write_all(format!("[tx {}] - {:?}", tx, action).as_bytes())
            .expect("FileLog.log: could not write to log file");
    }
}
