use serde::Serialize;

use super::{cpu::Cpu, disk::Disk, memory::Memory, primary::Primary, process::Process};

#[derive(Serialize)]
pub struct SysInfo {
    pub memory: Memory,
    pub primary: Primary,
    pub processes: Vec<Process>,
    pub disks: Vec<Disk>,
    pub cpus: Vec<Cpu>,
}

impl SysInfo {
    pub fn new(memory: Memory, primary: Primary, processes: Vec<Process>, disks: Vec<Disk>, cpus: Vec<Cpu>) -> Self {
        SysInfo {
            memory,
            primary,
            processes,
            disks,
            cpus
        }
    }
}
