use std::{thread, time::Duration};
use sysinfo::{Disks, System};
use crate::models::{
    cpu::Cpu, disk::Disk, disk_usage::DiskUsage, memory::Memory, primary::Primary,
    process::Process, sys_info::SysInfo,
};

fn load_cpu_usage(sys: &mut System) -> Vec<Cpu> {
    sys.refresh_cpu_all();
    thread::sleep(Duration::from_secs(1));
    sys.refresh_cpu_all();

    sys.cpus()
        .iter()
        .map(|cpu| {
            Cpu::new(
                cpu.name().to_string(),
                cpu.cpu_usage(),
                cpu.frequency(),
                cpu.vendor_id().to_string(),
                cpu.brand().to_string(),
            )
        })
        .collect()
}

fn load_memory_usage(sys: &mut System) -> Memory {
    sys.refresh_memory();
    Memory::new(
        sys.total_memory(),
        sys.used_memory(),
        sys.free_memory(),
        sys.total_swap(),
        sys.used_swap(),
        sys.free_swap(),
    )
}

fn load_processes(sys: &System) -> Vec<Process> {
    sys.processes()
        .iter()
        .map(|(pid, process)| {
            let disk_usage = process.disk_usage();
            Process::new(
                pid.as_u32(),
                process.name().to_string_lossy().into_owned(),
                DiskUsage::new(
                    disk_usage.total_written_bytes,
                    disk_usage.total_read_bytes,
                    disk_usage.written_bytes,
                    disk_usage.read_bytes,
                ),
            )
        })
        .collect()
}

fn load_disks() -> Vec<Disk> {
    Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| {
            Disk::new(
                disk.name().to_string_lossy().into_owned(),
                disk.file_system().to_string_lossy().into_owned(),
                disk.kind().to_string(),
                disk.is_removable(),
                disk.mount_point().to_string_lossy().into_owned(),
                disk.available_space(),
                disk.total_space(),
            )
        })
        .collect()
}

pub fn load() -> SysInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let memory = load_memory_usage(&mut sys);
    let cpus = load_cpu_usage(&mut sys);
    let processes = load_processes(&sys);
    let disks = load_disks();

    let primary = Primary::new(
        System::name().unwrap(),
        System::kernel_version().unwrap(),
        System::os_version().unwrap(),
        System::host_name().unwrap(),
        sys.processes().len(),
        sys.cpus().len(),
    );

    SysInfo::new(memory, primary, processes, disks, cpus)
}
