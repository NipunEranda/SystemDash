use sysinfo::{Components, Disks, Networks, System};

use crate::models::{disk_usage::DiskUsage, memory::{self, Memory}, primary::Primary, process::Process, sys_info::SysInfo};

pub fn load() -> SysInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let memory = Memory::new(sys.total_memory(), sys.used_memory(), sys.total_swap(), sys.used_swap());
    let primary = Primary::new(System::name().unwrap(), System::kernel_version().unwrap(), System::os_version().unwrap(), System::host_name().unwrap());
    let mut processes: Vec<Process> = vec![];


    for (pid, process) in sys.processes() {
        let disk_usage = process.disk_usage();
        processes.push(Process::new(pid.as_u32(), String::from(process.name().to_str().unwrap()), DiskUsage::new(disk_usage.total_written_bytes, disk_usage.total_read_bytes, disk_usage.written_bytes, disk_usage.read_bytes)));
    }

    // serde_json::to_string(&memory).unwrap()
    SysInfo::new(memory, primary, sys.cpus().len(), processes)
}
