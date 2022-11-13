use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Block<T> {
    pub prev_hash: Option<String>,
    pub items: Vec<T>,
    pub nonce: u32,
}

impl<T> Block<T>
where
    T: Clone + Serialize,
{
    pub fn new(prev_hash: Option<String>, items: &Vec<T>, nonce: u32) -> Block<T> {
        Block {
            prev_hash,
            items: items.to_vec(),
            nonce,
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        // Compute hash
        let block_serial = serde_json::to_string(self).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(block_serial.as_bytes());
        let hash = hasher.finalize().as_slice().to_vec();
        hash
    }

    pub fn is_valid(&self, difficulty: usize) -> bool {
        let block_hash = self.hash();
        let init_bytes = &block_hash[0..difficulty + 1];
        let mut response = true;
        for byte in init_bytes.iter() {
            if *byte != 0 {
                response = false;
                break;
            }
        }
        response
    }
}
