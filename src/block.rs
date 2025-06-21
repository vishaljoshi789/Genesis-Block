use std::{fmt::{ self, Debug, Formatter }};
use super::*;

pub struct Block{
    pub index: u32, 
    pub timestamp: u128,
    pub hash: BlockHash,
    pub previous_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block {{ index: {}, timestamp: {}, hash: {:?}, nonce: {}, payload: {} }}",
            self.index,
            self.timestamp,
            hex::encode(&self.hash),
            self.nonce,
            self.payload
        )
    }
}

impl Block {
    pub fn new(index: u32, previous_hash: BlockHash, payload: String) -> Self {
        Block {
            index,
            timestamp: now(),
            hash: vec![0; 32],
            previous_hash,
            nonce: 0,
            payload,
        }
    }

    pub fn set_hash(&mut self, hash: BlockHash) {
        self.hash = hash;
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(u32_bytes(&self.index));
        bytes.extend(u128_bytes(&self.timestamp));
        bytes.extend(&self.previous_hash);
        bytes.extend(u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());

        bytes
    }
}