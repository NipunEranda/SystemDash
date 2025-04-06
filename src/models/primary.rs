use serde::Serialize;

#[derive(Serialize)]
pub struct Primary {
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
    pub process_count: usize,
    pub cpu_count: usize,
}

impl Primary {
    pub fn new(system_name: String, kernel_version: String, os_version: String, host_name: String, process_count: usize, cpu_count: usize) -> Self {
        Primary {
            system_name,
            kernel_version,
            os_version,
            host_name,
            process_count,
            cpu_count
        }
    }
}