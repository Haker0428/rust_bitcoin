use chrono::prelude::*;
use utils::coder;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub timestamp: i64,
    pub parent_hash: String,
    pub tx_hash: String,
    pub nonce: u64,
    pub difficulty: u64,
    pub miner: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub transactions: String,
}

impl Block {
    pub fn set_hash(&mut self) {
        let header = coder::my_serialze(&self.header);
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String, pre_hash: String) -> Block {
        let transactions = coder::my_serialze(&data);
        let tx_hash = coder::get_hash(&transactions[..]);
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            header: BlockHeader {
                timestamp: timestamp,
                parent_hash: pre_hash,
                tx_hash: tx_hash,
                nonce: 0,
                difficulty: 0,
                miner: "".to_string(),
            },
            hash: "".to_string(),
            transactions: data
        };
        block.set_hash();
        block
    }
}
