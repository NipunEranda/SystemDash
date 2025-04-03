use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    index: String,
    usage: f32,
    frequency: u64,
    vendor_id: String,
    brand: String,
}

impl Cpu {
    pub fn new(
        index: String,
        usage: f32,
        frequency: u64,
        vendor_id: String,
        brand: String,
    ) -> Self {
        Cpu {
            index,
            usage,
            frequency,
            vendor_id,
            brand
        }
    }
}
