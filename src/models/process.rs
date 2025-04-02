use serde::{Deserialize, Serialize};

use super::disk_usage::DiskUsage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    id: u32,
    name: String,
    disk_usage: DiskUsage,
}

impl Process {
    pub fn new(id: u32, name: String, disk_usage: DiskUsage) -> Self {
        Process {
            id,
            name,
            disk_usage
        }
    }
}