use sysinfo::{Components, System};

// declare module
mod device;

// use module
use device::Device;

fn main() {
    let mut device = Device {
        name: String::from("Raspberry Pi 5"),
        components: Components::new_with_refreshed_list(),
        system: System::new_all(),
    };

    println!("Device name: {}", device.name);

    device.display_average_temperature();
    device.display_average_cpu_usage();

}
