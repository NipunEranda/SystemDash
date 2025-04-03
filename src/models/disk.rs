use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    name: String,
    file_type: String,
    disk_type: String,
    is_removable: bool,
    mount_point: String,
    available_space: u64,
    total_space: u64,
}

impl Disk {
    pub fn new(
        name: String,
        file_type: String,
        disk_type: String,
        is_removable: bool,
        mount_point: String,
        available_space: u64,
        total_space: u64,
    ) -> Self {
        Disk {
            name,
            file_type,
            disk_type,
            is_removable,
            mount_point,
            available_space,
            total_space,
        }
    }
}
