use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskUsage {
    pub total_written: u64,
    pub total_read: u64,
    pub write: u64,
    pub read: u64,
}

impl DiskUsage {
    pub fn new(total_written: u64, total_read: u64, write: u64, read: u64) -> Self {
        DiskUsage {
            total_written,
            total_read,
            write,
            read,
        }
    }
}