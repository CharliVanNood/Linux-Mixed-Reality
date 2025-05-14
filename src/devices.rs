use rusb::{Context, UsbContext};
use std::collections::{HashMap, HashSet};

use crate::devices;

pub struct Devices {
    devices: HashSet<(u8, u8, u16, u16)>,
    selected_devices: Vec<((u8, u8, u16, u16), Vec<u32>)>
}
impl Devices {
    pub fn new() -> Self {
        Self {
            devices: HashSet::new(),
            selected_devices: Vec::new()
        }
    }

    pub fn check_devices(&mut self) {
        let context = Context::new().unwrap();
        let devices = context.devices().unwrap();

        print!("\n\n\n\n\n\n\n\n");
        for device in devices.iter() {
            let desc = device.device_descriptor().unwrap();

            if !self.devices.contains(&(device.bus_number(), device.address(), desc.vendor_id(), desc.product_id())) {
                self.devices.insert((device.bus_number(), device.address(), desc.vendor_id(), desc.product_id()));

                if desc.vendor_id() == 0x048d && desc.product_id() == 0xc981 {
                    self.selected_devices.push(((device.bus_number(), device.address(), desc.vendor_id(), desc.product_id()), Vec::new()))
                }
            }

            println!(
                "Bus {:03} Device {:03} ID {:04x}:{:04x} Raw: {:?}",
                device.bus_number(),
                device.address(),
                desc.vendor_id(),
                desc.product_id(),
                device.as_raw()
            );
        }

        println!("-------------------- {} Devices -------------------", devices.len());
    }

    pub fn check_usb(&mut self) {
        let context = Context::new().unwrap();
        let devices = context.devices().unwrap();

        for device_checking in devices.iter() {
            let desc = device_checking.device_descriptor().unwrap();

            for device_item in &self.selected_devices {
                if device_item.0.0 != device_checking.bus_number() ||
                    device_item.0.1 != device_checking.address() ||
                    device_item.0.2 != desc.vendor_id() ||
                    device_item.0.3 != desc.product_id() { continue; }
                let device = &device_checking;
                println!("data: {:?}", device.as_raw())
            }
        }
    }
}
