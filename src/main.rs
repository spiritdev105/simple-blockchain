use sha2::{Sha256, Digest}; // We'll use the SHA256 function here.
use std::time::{SystemTime, UNIX_EPOCH}; // For getting the time.

#[derive(Debug)] // Debug trait for printing
#[allow(dead_code)] // Suppress warnings for unused code
struct Block {
    index: u32, // Represents the position of the block in the blockchain.
    timestamp: u128, // Represents the timestamp of when the block was created.
    data: String, // Represents the data to be stored in the block.
    previous_hash: String, // Stores the hash of the previous block.
    hash: String, // Stores the hash of this block, generated using the SHA256 function.
}

impl Block { 
    // Constructor function for creating a new block
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        
        Block { index, timestamp, data, previous_hash, hash }
    }

    // Function to calculate the SHA256 hash of the block
    fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        format!("{:x}", hasher.finalize()) // Return hash in hexadecimal format
    }
}

#[derive(Debug)] // Debug trait for blockchain
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain { 
    // Creates a new blockchain with the genesis block
    fn new() -> Blockchain {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

    // Function to add a new block to the blockchain
    fn add_block(&mut self, data: String) {
        let index = self.chain.len() as u32;
        let previous_hash = if index == 0 { 
            "0".to_string() 
        } else { 
            self.chain[index as usize - 1].hash.clone() 
        };
        let block = Block::new(index, data, previous_hash);
        self.chain.push(block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First real block.".to_string());
    blockchain.add_block("Second block.".to_string());

    for block in &blockchain.chain { 
        println!("{:?}", block);
    }
}
