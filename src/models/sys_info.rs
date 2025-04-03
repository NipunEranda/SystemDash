use serde::Serialize;

use super::{disk::Disk, memory::Memory, primary::Primary, process::Process};

#[derive(Serialize)]
pub struct SysInfo {
    pub memory: Memory,
    pub primary: Primary,
    pub nb_cpus: usize,
    pub processes: Vec<Process>,
    pub disks: Vec<Disk>,
}

impl SysInfo {
    pub fn new(memory: Memory, primary: Primary, nb_cpus: usize, processes: Vec<Process>, disks: Vec<Disk>) -> Self {
        SysInfo {
            memory,
            primary,
            nb_cpus,
            processes,
            disks
        }
    }
}
