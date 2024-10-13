use sha2::{Sha256, Digest}; // We'll use the SHA256 fuction here.
use std::time::{SystemTime, UNIX_EPOCH}; //  For getting the time.

#[derive(Debug)] // Debug
#[allow(dead_code)] 
struct Block {
    index: u32, // Represents the number of bytes in the block.
    timestamp: u128, // Represents the timestamp of when the block was created.
    data: String, // Represents the the data to be stored in the block.
    previous_hash: String, // Stores the hash of the previous block.
    hash: String, // Stores the hash of this block, generated using the SHA256 function.
}

impl Block 
