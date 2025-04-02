use serde::Serialize;

#[derive(Serialize)]
pub struct Primary {
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String
}

impl Primary {
    pub fn new(system_name: String, kernel_version: String, os_version: String, host_name: String) -> Self {
        Primary {
            system_name,
            kernel_version,
            os_version,
            host_name,
        }
    }
}