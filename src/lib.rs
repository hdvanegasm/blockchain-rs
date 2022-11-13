mod block;

use hex;
use serde::Serialize;
use block::Block;

struct Blockchain<T> {
    blocks: Vec<Block<T>>,
    difficulty: usize,
}

impl<T> Blockchain<T>
where
    T: Clone + Serialize,
{
    fn new(difficulty: usize) -> Blockchain<T> {
        Blockchain {
            blocks: vec![],

            // TODO Correct the difficulty to be bits and not bytes
            difficulty,
        }
    }

    fn add_block(&mut self, mut block: Block<T>) {
        if self.blocks.len() == 0 {
            block.prev_hash = Option::None;
        } else {
            block.prev_hash = Some(hex::encode(self.blocks[self.blocks.len() - 1].hash()));
        }
        self.mine_block(&mut block);
        self.blocks.push(block);
    }

    fn mine_block(&self, block: &mut Block<T>) -> u32 {
        let mut curr_nonce: u32 = 0;

        loop {
            // Serialize block with new nonce
            block.nonce = curr_nonce;

            if block.is_valid(self.difficulty) {
                return curr_nonce;
            }

            curr_nonce += 1;
        }
    }

    pub fn is_valid(&self) -> bool {
        for block in &self.blocks {
            if !block.is_valid(self.difficulty) {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_block() {
        let difficulty: usize = 1;
        let block = Block::new(
            Option::None,
            &vec![1, 2, 3],
            0,
        );

        let mut blockchain: Blockchain<i32> = Blockchain::new(
            difficulty,
        );

        blockchain.add_block(block);
        let hash_block = blockchain.blocks[0].hash();
        let initial_bytes = &hash_block[..difficulty + 1];
        for byte in initial_bytes {
            assert_eq!(*byte, 0);
        }
    }
}