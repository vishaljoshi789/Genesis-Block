use std::vec;
use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(1, vec![0; 32], 0, "Genesis Block".to_string(), difficulty);

    block.mine();
    println!("Mined genesis block {:?}", block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block]
    };

    if blockchain.verify() {
        println!("Verify: True");
    } else {
        println!("Verify: False");
        return;
    }

    for i in 1..=10 {
        let mut block = Block::new(i, last_hash, 0, "Another Block".to_string(), difficulty);

        block.mine();
        println!("New block {:?}", block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);

        if blockchain.verify() {
            println!("Verify: True");
        } else {
            println!("Verify: False");
            break;
        }
    }
}
