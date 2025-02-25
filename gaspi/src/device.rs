use sysinfo::{Components, System};

pub struct Device {
    pub name: String,
    pub components: Components,
    pub system: System,
}

impl Device {
    /*
    pub fn new(name: String, components: Components ) -> Device {
        Device { name, components }
    }
    */

    pub fn average_temperature(&mut self) -> f32 {
        self.components.refresh(true);
        let mut total_temp = 0.0;
        let mut count = 0;

        for component in self.components.list() {
            if let Some(temp) = component.temperature() {
                //println!("{}: {:.2}°C", component.label(), temp);
                total_temp += temp;
                count += 1;
            }
        }

        if count > 0 {
            let average_temp = total_temp / count as f32;
            average_temp
        } else {
            // No temperature sensors detected.
            0.0
        }
    }
    pub fn display_average_temperature(&mut self) {
        let t = self.average_temperature();
        if t > 0.0 {
            println!("🌡️ Average Workstation Temperature: {:.2}°C", t);
        } else {
            println!("✋ No temperature sensors detected.");
        }

    }

    pub fn average_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu_usage();
        let cpus = self.system.cpus();
        let total_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        let cpu_count = cpus.len() as f32;

        if cpu_count > 0.0 {
            let average_usage = total_usage / cpu_count;
            average_usage
        } else {
            // No CPU data available.
            0.0
        }
    }

    pub fn display_average_cpu_usage(&mut self) {
        let u = self.average_cpu_usage();
        if u > 0.0 {
            println!("📶 Average CPU Consumption: {:.2}%", u);
        } else {
            println!("✋ No CPU data available.");
        }
    }

}

