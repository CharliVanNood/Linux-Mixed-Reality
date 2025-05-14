use std::{thread, time::Duration};

mod devices;

fn main() {
    let mut devices = devices::Devices::new();

    let mut check_index = 0;
    loop {
        if check_index % 500 == 0 { devices.check_devices(); }
        devices.check_usb();

        check_index += 1;
        thread::sleep(Duration::from_millis(1));
    }
}