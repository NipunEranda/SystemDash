use sysinfo::{Components, Disks, System};

use crate::models::{
    cpu::Cpu, disk::Disk, disk_usage::DiskUsage, memory::Memory, primary::Primary,
    process::Process, sys_info::SysInfo,
};

pub fn load() -> SysInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let memory = Memory::new(
        sys.total_memory(),
        sys.used_memory(),
        sys.free_memory(),
        sys.total_swap(),
        sys.used_swap(),
        sys.free_swap(),
    );
    let primary = Primary::new(
        System::name().unwrap(),
        System::kernel_version().unwrap(),
        System::os_version().unwrap(),
        System::host_name().unwrap(),
        sys.processes().len(),
        sys.global_cpu_usage(),
        sys.cpus().len()
    );
    let mut processes: Vec<Process> = vec![];
    let mut disks: Vec<Disk> = vec![];
    let mut cpus: Vec<Cpu> = vec![];

    for (pid, process) in sys.processes() {
        let disk_usage = process.disk_usage();
        processes.push(Process::new(
            pid.as_u32(),
            String::from(process.name().to_str().unwrap()),
            DiskUsage::new(
                disk_usage.total_written_bytes,
                disk_usage.total_read_bytes,
                disk_usage.written_bytes,
                disk_usage.read_bytes,
            ),
        ));
    }

    for disk in &Disks::new_with_refreshed_list() {
        // println!("Name: {} File Type: {} Disk Type: {} Is Removable: {} Mount Point: {} Available Space: {} Total Space: {}", disk.name().to_str().unwrap(), disk.file_system().to_str().unwrap(), disk.kind(), disk.is_removable(), disk.mount_point().to_str().unwrap(), disk.available_space(), disk.total_space());
        disks.push(Disk::new(
            String::from(disk.name().to_str().unwrap()),
            String::from(disk.file_system().to_str().unwrap()),
            String::from(disk.kind().to_string()),
            disk.is_removable(),
            String::from(disk.mount_point().to_str().unwrap()),
            disk.available_space(),
            disk.total_space(),
        ));
    }

    for cpu in sys.cpus() {
        cpus.push(Cpu::new(
            cpu.name().to_string(),
            cpu.cpu_usage(),
            cpu.frequency(),
            cpu.vendor_id().to_string(),
            cpu.brand().to_string(),
        ));
    }

    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }

    // serde_json::to_string(&memory).unwrap()
    SysInfo::new(memory, primary, processes, disks, cpus)
}
