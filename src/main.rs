use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
#[allow(dead_code)]
struct Block {
    index: u32, // Position in the blockchain.
    timestamp: u128, // Block creation time.
    data: String, // Data stored in the blockchain.
    previous_hash: String, // Hash of previous block.
    hash: String, // Hash of this block.
    nonce: u64, // Random number used for mining.
}

impl Block {
    // Create a new block.
    fn new(index: u32, data: String, previous_hash: String, difficulty: u32) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let (nonce, hash) = Block::mine_block(index, timestamp, &data, &previous_hash, difficulty);
        
        Block { index, timestamp, data, previous_hash, hash, nonce }
    }

    // SHA256 hash hesaplaması
    fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        hasher.update(nonce.to_string());
        format!("{:x}", hasher.finalize()) // Returns the hash in hexadecimal format.
    }

    // Blok için madencilik (proof of work)
    fn mine_block(index: u32, timestamp: u128, data: &str, previous_hash: &str, difficulty: u32) -> (u64, String) {
        let target_prefix = "0".repeat(difficulty as usize); // The hash must start with this many zeros.
        let mut nonce = 0;
        
        loop {
            let hash = Block::calculate_hash(index, timestamp, data, previous_hash, nonce);
            if hash.starts_with(&target_prefix) {
                return (nonce, hash); // Return when a suitable hash is found.
            }
            nonce += 1; // Increment the nonce.
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>, // The blockchain.
    difficulty: u32, // Mining difficulty level.
}

impl Blockchain {
    // Create a new blockchain.
    fn new(difficulty: u32) -> Blockchain {
        let mut blockchain = Blockchain { chain: Vec::new(), difficulty };
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

    // Blok ekleme işlemi
    fn add_block(&mut self, data: String) {
        let index = self.chain.len() as u32;
        let previous_hash = if index == 0 { 
            "0".to_string() 
        } else { 
            self.chain[index as usize - 1].hash.clone() 
        };
        let block = Block::new(index, data, previous_hash, self.difficulty);
        self.chain.push(block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new(4); // Difficulty level 4. 4 a low level for difficulty.
    blockchain.add_block("First real block.".to_string());
    blockchain.add_block("Second block.".to_string());

    for block in &blockchain.chain { 
        println!("{:?}", block);
    }
}
