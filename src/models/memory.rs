use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,
}

impl Memory {
    pub fn new(total_memory: u64, used_memory: u64, free_memory: u64, total_swap: u64, used_swap: u64, free_swap: u64) -> Self {
        Memory {
            total_memory,
            used_memory,
            free_memory,
            total_swap,
            used_swap,
            free_swap
        }
    }
}