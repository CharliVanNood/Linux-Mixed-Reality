use rusb::{Context, UsbContext};
use std::collections::HashSet;

pub struct Devices {
    devices: HashSet<(u8, u8, u16, u16)>
}
impl Devices {
    pub fn new() -> Self {
        Self { devices: HashSet::new() }
    }

    pub fn check_devices(&mut self) {
        let context = Context::new().unwrap();

        let devices = context.devices().unwrap();
        let mut devices_len = 0;

        for device in devices.iter() {
            let desc = device.device_descriptor().unwrap();

            if !self.devices.contains(&(device.bus_number(), device.address(), desc.vendor_id(), desc.product_id())) {
                self.devices.insert((device.bus_number(), device.address(), desc.vendor_id(), desc.product_id()));

                println!(
                    "Bus {:03} Device {:03} ID {:04x}:{:04x}",
                    device.bus_number(),
                    device.address(),
                    desc.vendor_id(),
                    desc.product_id()
                );

                devices_len += 1;
            }
        }

        if devices_len > 0 {
            println!("Found {} devices", devices.len());
        }
    }
}
