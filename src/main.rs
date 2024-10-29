use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

// Block struct with added salt for hash uniqueness
#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
    salt: String, // Added salt value
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String, difficulty: u32) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let salt = format!("{}{}", index, timestamp); // Generate salt from index and timestamp
        let (nonce, hash) = Block::mine_block(index, timestamp, &data, &previous_hash, difficulty, &salt);
        
        Block { index, timestamp, data, previous_hash, hash, nonce, salt }
    }

    // Calculate hash with added salt
    fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str, nonce: u64, salt: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        hasher.update(nonce.to_string());
        hasher.update(salt); // Include salt in the hash calculation
        format!("{:x}", hasher.finalize())
    }

    // Improved mine_block function with nonce limit for performance optimization
    fn mine_block(index: u32, timestamp: u128, data: &str, previous_hash: &str, difficulty: u32, salt: &str) -> (u64, String) {
        let target_prefix = "0".repeat(difficulty as usize);
        let mut nonce = 0;

        loop {
            let hash = Block::calculate_hash(index, timestamp, data, previous_hash, nonce, salt);
            if hash.starts_with(&target_prefix) {
                return (nonce, hash);
            }
            nonce += 1;
            if nonce > 1_000_000 { // Example nonce limit to avoid long mining times
                println!("Nonce limit reached, restarting mining process.");
                nonce = 0;
            }
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
}

impl Blockchain {
    fn new(difficulty: u32) -> Blockchain {
        let mut blockchain = Blockchain { chain: Vec::new(), difficulty };
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

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

    fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                println!("Error: Block {} has an invalid previous hash!", i);
                return false;
            }
            let calculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
                current.nonce,
                &current.salt, // Added salt here
            );
            if current.hash != calculated_hash {
                println!("Error: Block {} hash does not match calculated hash!", i);
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new(4);
    blockchain.add_block("First real block.".to_string());
    blockchain.add_block("Second Block.".to_string());

    if blockchain.validate_chain() {
        println!("Blockchain valid.");
    } else {
        println!("Blockchain invalid.");
    }

    for block in &blockchain.chain {
        println!("{:?}", block);
    }
}
