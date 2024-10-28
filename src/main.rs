use sha2::{Sha256, Digest}; // Import the SHA-256 cryptographic algorithm. Digest helps with hash computation.
use std::time::{SystemTime, UNIX_EPOCH}; // Necessary libraries for obtaining UNIX timestamps.

#[derive(Debug)] // Adds the Debug trait so that we can easily print `Block` values using `println!("{:?}", block);`.
#[allow(dead_code)] // Suppresses compiler warnings for unused code.
struct Block {
    index: u32,            // Holds the sequence number of each block.
    timestamp: u128,       // The time the block was created (in milliseconds since UNIX epoch).
    data: String,          // The data to be stored within the block.
    previous_hash: String, // The hash of the previous block.
    hash: String,          // The hash of this block.
    nonce: u64,            // The nonce value used for Proof-of-Work.
}

impl Block {
    // The `new` function creates a new `Block`.
    fn new(index: u32, data: String, previous_hash: String, difficulty: u32) -> Block { 
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(); // Get the current time.
        let (nonce, hash) = Block::mine_block(index, timestamp, &data, &previous_hash, difficulty); // Calculate nonce and hash.
        
        // Return a new `Block`.
        Block { index, timestamp, data, previous_hash, hash, nonce }
    }

    // This function calculates the hash based on the block's contents.
    fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new(); // Create a new SHA-256 hasher instance.
        hasher.update(index.to_string());      // Add the block's index.
        hasher.update(timestamp.to_string());   // Add the timestamp.
        hasher.update(data);                    // Add the data.
        hasher.update(previous_hash);           // Add the previous block's hash.
        hasher.update(nonce.to_string());       // Add the nonce value.
        format!("{:x}", hasher.finalize()) // Return the hash in hexadecimal format.
    }

    // The `mine_block` function generates a block at a given difficulty level.
    fn mine_block(index: u32, timestamp: u128, data: &str, previous_hash: &str, difficulty: u32) -> (u64, String) {
        let target_prefix = "0".repeat(difficulty as usize); // The target hash must start with a specific number of '0's.
        let mut nonce = 0; // Set the starting nonce value to zero.

        loop {
            let hash = Block::calculate_hash(index, timestamp, data, previous_hash, nonce); // Compute the hash.
            if hash.starts_with(&target_prefix) { // If the hash meets the target, return the nonce and hash.
                return (nonce, hash);
            }
            nonce += 1; // If not, increment the nonce and try again.
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>, // A list of blocks that make up the blockchain.
    difficulty: u32,   // The difficulty level for Proof-of-Work.
}

impl Blockchain {
    // The `new` function initializes a new `Blockchain` and adds the genesis block.
    fn new(difficulty: u32) -> Blockchain {
        let mut blockchain = Blockchain { chain: Vec::new(), difficulty };
        blockchain.add_block("Genesis Block".to_string()); // Add the first block ("Genesis Block").
        blockchain
    }

    // The `add_block` function appends a new block to the chain.
    fn add_block(&mut self, data: String) {
        let index = self.chain.len() as u32; // Set the block's index based on the current length of the chain.
        let previous_hash = if index == 0 { 
            "0".to_string() // If it's the first block, set previous hash to "0".
        } else {
            self.chain[index as usize - 1].hash.clone() // Otherwise, get the hash from the previous block.
        };
        let block = Block::new(index, data, previous_hash, self.difficulty); // Create a new block.
        self.chain.push(block); // Append the block to the chain.
    }

    // The `validate_chain` function checks the integrity of the blockchain.
    fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i]; // Get the current block.
            let previous = &self.chain[i - 1]; // Get the previous block.

            // Check if the previous hash of the current block matches the hash of the previous block.
            if current.previous_hash != previous.hash {
                println!("Error: Block {} has a wrong previous hash value!", i);
                return false; // If not, the chain is invalid.
            }

            // Recalculate the hash based on the current block's contents.
            let calculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.data,
                &current.previous_hash,
                current.nonce,
            );
            // Verify if the current block's hash matches the calculated hash.
            if current.hash != calculated_hash {
                println!("Error: Block {} hash value not confirmed!", i);
                return false; // If not, the chain is invalid.
            }
        }
        true // If all checks pass, the chain is valid.
    }
}

fn main() {
    let mut blockchain = Blockchain::new(4); // Initialize a new blockchain with a difficulty of 4.
    blockchain.add_block("First real block.".to_string()); // Add the first real block.
    blockchain.add_block("Second Block.".to_string()); // Add the second block.

    // Validate the chain and print the result.
    if blockchain.validate_chain() {
        println!("Blockchain valid.");
    } else {
        println!("Blockchain invalid.");
    }

    // Print all blocks in the chain.
    for block in &blockchain.chain {
        println!("{:?}", block);
    }
}
