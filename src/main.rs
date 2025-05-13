use std::{thread, time::Duration};

mod devices;

fn main() {
    let mut devices = devices::Devices::new();

    loop {
        devices.check_devices();
        thread::sleep(Duration::from_millis(500));
    }
}