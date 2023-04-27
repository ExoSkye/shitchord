use std::io::BufWriter;
use ciborium::{de, ser};
use ciborium::value::Value;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};
use zstd::{Decoder, Encoder};
use zstd::stream::zio::Writer;
use crate::chord::GUID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NodeMessage {
    Lookup(GUID),
    Data(GUID, Value),
    TraceRoute(GUID)
}

impl NodeMessage {
    pub fn serialize(&self, level: Option<i32>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let mut zstd_writer = Encoder::new(&mut output, level.unwrap_or(3)).unwrap();
        ser::into_writer(self, &mut zstd_writer).unwrap();

        output
    }
}

impl From<Vec<u8>> for NodeMessage {
    fn from(value: Vec<u8>) -> Self {
        let zstd_reader = Decoder::new(value.as_slice()).unwrap();
        de::from_reader(zstd_reader).unwrap()
    }
}

pub trait Node {
    fn get_channel() -> (Sender<NodeMessage>, Receiver<NodeMessage>);
}