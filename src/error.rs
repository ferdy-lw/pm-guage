use std::{
    sync::{
        mpsc::{self, SyncSender},
        Arc, LazyLock, Mutex,
    },
    thread,
};

use circular_buffer::CircularBuffer;
use log::error;
use thiserror::Error;

use crate::ui::STATUS;

#[derive(Error, Debug)]
pub enum ReadObdError {
    #[error("Bad command response ({response}) length ({length})")]
    BadCommandResponseLength { response: String, length: usize },

    #[error("Bad hex string ({hex_str}) in command response ({response})")]
    BadCommandResponseHex {
        response: String,
        hex_str: String,
        source: std::num::ParseIntError,
    },

    #[error("Unsupported command response ({response})")]
    BadCommandResponseUnsupported { response: String },

    #[error("Device Read Error")]
    ReadError(#[from] std::io::Error),

    #[error("Error posting request to elm327")]
    HttpPostError,
}

pub static MSG_LOGGER: LazyLock<MsgLogger> = LazyLock::new(MsgLogger::start);

pub struct MsgLogger {
    tx: SyncSender<String>,
    messages: Arc<Mutex<CircularBuffer<10, String>>>,
}

impl MsgLogger {
    pub fn log(&self, msg: String) {
        let _ = self.tx.send(msg);
    }

    pub fn get_messages(&self) -> String {
        let mut messages = String::new();

        messages += STATUS
            .read()
            .unwrap()
            .as_ref()
            .map_or("", |s| s.to_str().unwrap_or(""));

        for msg in self.messages.lock().unwrap().iter() {
            messages += msg;
            messages.push_str("\r\n");
        }

        messages
    }

    fn start() -> Self {
        let (tx, rx) = mpsc::sync_channel(10);

        let mut buffer = CircularBuffer::new();
        buffer.push_back("Start of log".to_owned());

        let messages = Arc::new(Mutex::new(buffer));

        let messages1 = Arc::clone(&messages);
        thread::spawn(move || loop {
            match rx.recv() {
                Ok(msg) => {
                    messages1.lock().unwrap().push_back(msg);
                }
                Err(e) => {
                    error!("Message logger sender disconnected? {e}");
                    break;
                }
            }
        });

        MsgLogger { tx, messages }
    }
}
