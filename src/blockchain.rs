use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
         for (i, block) in self.blocks.iter().enumerate(){
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", i, block.index);
                return false;
            }
            else if !block::check_difficulty(&block.hash()
                , block.difficulty) {
                println!("Difficulty Failed for block");
                return false;
            }
            else if i != 0 {
                let previous_block = &self.blocks[i - 1];
                if block.timestamp <= previous_block.timestamp {
                    println!("Timestamp is not greater than previous block");
                    return false;
                } else if block.previous_hash != previous_block.hash {
                    println!("Previous hash mismatch");
                    return false;
                }
            }
            else{
                if block.previous_hash != vec![0; 32] {
                    println!("Genesis block has non-zero previous hash");
                    return false;
                }
            }
        }
        return true;
    }
}