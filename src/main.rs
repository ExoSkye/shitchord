use ciborium::cbor;
use ciborium::value::Value;
use serde::Serialize;
use crate::chord::{GUID, NodeMessage};

mod chord;

fn main() {
    let message = NodeMessage::Data(GUID::new(), cbor!((0..1000000).into_iter().collect::<Vec<u128>>()).unwrap());

    for i in 1..21 {
        let msg = message.serialize(Some(i));

        println!("{}", msg.len());
    }
}
