use rusb::{Context, UsbContext};

fn main() {
    let context = Context::new().unwrap();

    let devices = context.devices().unwrap();
    println!("Found {} devices", devices.len());

    for device in devices.iter() {
        let desc = device.device_descriptor().unwrap();

        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            desc.vendor_id(),
            desc.product_id()
        );
    }
}