use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};

use crate::config::GeneralConfig;

pub struct Logger {
    handler: JoinHandle<()>,
    tx: Sender<String>,
}
#[derive(Clone)]
pub struct LoggerSender {
    tx: Sender<String>,
}

impl Logger {
    pub fn from_path(path: &str) -> Result<Logger, Box<dyn Error>> {
        let data = std::fs::read_to_string(path)?;
        let config: GeneralConfig = serde_json::from_str(&data)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(config.logger_path)
            .expect("[CRITICAL] Cannot open file");

        let (tx, rx): (Sender<String>, Receiver<String>) = channel();

        let join_handler = spawn(move || {
            while let Ok(msg) = rx.recv() {
                let c = msg.clone() + "\n";
                print!("{}", c);
                file.write_all(c.as_bytes())
                    .expect("[CRITICAL] Write to file failed");
            }
        });

        let logger = Logger {
            handler: join_handler,
            tx,
        };

        Ok(logger)
    }
    pub fn get_sender(&self) -> LoggerSender {
        LoggerSender {
            tx: self.tx.clone(),
        }
    }
    pub fn close(self) {
        drop(self.tx);
        let _ret = self.handler.join();
    }
}

impl LoggerSender {
    pub fn send(&self, msg: String) {
        let _ = self.tx.send(msg);
    }
}
