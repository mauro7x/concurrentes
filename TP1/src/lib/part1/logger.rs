use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

use crate::common::{config::LoggerConfig, utils};

pub struct Logger {
    handler: JoinHandle<()>,
    tx: Sender<String>,
}
#[derive(Clone)]
pub struct LoggerSender {
    tx: Sender<String>,
}

impl Logger {
    pub fn from_config(config: LoggerConfig) -> Result<Logger, Box<dyn Error>> {
        let path = format!("{}/part1-{}.txt", config.dirpath, utils::now_rfc());
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("[CRITICAL] Error while opening logger file");

        let (tx, rx): (Sender<String>, Receiver<String>) = channel();

        let join_handler = spawn(move || Logger::write_to_log(rx, file));

        let logger = Logger {
            handler: join_handler,
            tx,
        };

        Ok(logger)
    }
    fn write_to_log(rx: Receiver<String>, mut file: File) {
        while let Ok(msg) = rx.recv() {
            let c = msg.clone() + "\n";
            // print!("{}", c);
            file.write_all(c.as_bytes())
                .expect("[CRITICAL] Write to file failed");
        }
    }
    pub fn get_sender(&self) -> LoggerSender {
        LoggerSender {
            tx: self.tx.clone(),
        }
    }
    pub fn join(self) {
        drop(self.tx);
        self.handler
            .join()
            .expect("[CRITICAL] Error joining logger thread");
    }
}

impl LoggerSender {
    pub fn send(&self, msg: String) {
        let _ = self.tx.send(msg);
    }
}
