use sysinfo::{Components, System};

// declare module
mod device;

// use module
use device::Device;
use device::PowerModel;

#[tokio::main] // Add this attribute to use async main
async fn main() {
    let mut device = Device {
        name: String::from("Raspberry Pi 5"),
        components: Components::new_with_refreshed_list(),
        system: System::new_all(),
        power_model: PowerModel {
            base_power: 5.0,        // Minimum power in watts (idle)
            max_power: 15.0,        // Maximum power in watts (full load)
            ram_power_factor: 0.25, // Higher impact of RAM on Pi (// The Pi 5's RAM is shared with the GPU, which affects power consumption)
            // Environmental factors for France
            // These values remain the same as they depend on the energy mix
            emission_factor: 60.0,          // gCO2eq/kWh
            abiotic_factor: 1.39e-6,        // kgSbeq/kWh
            primary_energy_factor: 10230.0, // kJ/kWh
        },
    };

    println!("🖥️ Device name: {}", device.name);

    //device.display_average_temperature();
    //device.display_average_cpu_usage();
    //device.display_virtual_memory_usage();
    //device.display_power_consumption();

    // monitoring
    println!("🔋 Monitoring device for 5 seconds...\n");
    device.monitor(5);

    // monitoring completion
    println!("🔋 Monitoring 🦙 Ollama completion [qwen2.5:7b]...\n");

    if let Err(e) = device.monitor_completion("http://localhost:11434", "qwen2.5:7b", "Who is Jean-Luc Picard?").await {
        eprintln!("Error: {:?}", e);
    }

    println!("🔋 Monitoring 🦙 Ollama completion [qwen2.5:14b]...\n");


    if let Err(e) = device.monitor_completion("http://localhost:11434", "qwen2.5:14b", "Who is Jean-Luc Picard?").await {
        eprintln!("Error: {:?}", e);
    }


}
