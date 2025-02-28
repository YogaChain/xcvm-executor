use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrossChainMessage {
    pub source_chain: String,
    pub destination_chain: String,
    pub payload: String,
}

pub struct Relayer;

impl Relayer {
    pub fn relay_message(msg: CrossChainMessage) {
        println!(
            "Relaying message from {} to {}: {:?}",
            msg.source_chain, msg.destination_chain, msg.payload
        );
    }
}
