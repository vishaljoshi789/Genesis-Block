use std::vec;

use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, vec![0; 32], "Genesis Block".to_string());

    println!("{:?}", block);

    let hash = block.hash();
    block.set_hash(hash);

    println!("{:?}", block);
}
