use serde::Serialize;

use super::{memory::Memory, primary::Primary, process::Process};

#[derive(Serialize)]
pub struct SysInfo {
    pub memory: Memory,
    pub primary: Primary,
    pub nb_cpus: usize,
    pub processes: Vec<Process>,
}

impl SysInfo {
    pub fn new(memory: Memory, primary: Primary, nb_cpus: usize, processes: Vec<Process>) -> Self {
        SysInfo {
            memory,
            primary,
            nb_cpus,
            processes
        }
    }
}
