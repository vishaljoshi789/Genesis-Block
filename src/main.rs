use std::vec;
use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, vec![0; 32], 0, "Genesis Block".to_string(), 0x00000fffffffffffffffffffffffffff);

    let hash = block.hash();
    block.set_hash(hash);
    println!("{:?}", block);

    block.mine();
    println!("{:?}", block);
}
