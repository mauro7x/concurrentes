//! System logging entities.

use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{spawn, JoinHandle},
};

use crate::common::{config::LoggerConfig, utils};

/// Logger is an entity that keeps a reference to the thread
/// that handles the logging. The thread writes to a file and prints to stdout
/// each log that processes. A channel is used for communication.

pub struct Logger {
    handler: JoinHandle<()>,
    tx: Sender<String>,
}

/// LoggerSender holds a reference to the channel that handles the communication
/// with the logger thread.

#[derive(Clone)]
pub struct LoggerSender {
    tx: Sender<String>,
}

impl Logger {
    /// Given a LoggerConfig this method will create a Logger entity, open the associated file and spawn a thread.

    pub fn from_config(LoggerConfig { dirpath }: LoggerConfig) -> Result<Logger, Box<dyn Error>> {
        fs::create_dir_all(&dirpath)?;
        let path = format!("{}/part1-{}.txt", dirpath, utils::now_rfc());
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
            let c = format!("[{}] {}\n", utils::now_h_m_s(), msg);
            print!("{}", c);
            file.write_all(c.as_bytes())
                .expect("[CRITICAL] Write to file failed");
        }
    }
    /// Get Sender copy for Logger communication channel.

    pub fn get_sender(&self) -> LoggerSender {
        LoggerSender {
            tx: self.tx.clone(),
        }
    }
    /// Join thread responsible for logging.

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
