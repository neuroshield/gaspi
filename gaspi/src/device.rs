use serde_json::Value;
use sysinfo::{Components, System};
use reqwest::Error;

pub struct EnergyStats {
    energy_wh: f32,
    ghg_emissions: f32,
    abiotic_resources: f32,
    primary_energy: f32,
    average_power: f32,
}

// The Raspberry Pi 5 has a base consumption of about 5W at idle
// Under maximum load, it can reach 12-15W depending on connected peripherals
pub struct PowerModel {
    pub base_power: f32, // Minimum power in watts (idle)
    pub max_power: f32,  // Maximum power in watts (full load)
    //ram_total: u64,        // Total memory in bytes
    pub ram_power_factor: f32,      // Higher impact of RAM on Pi
    pub emission_factor: f32,       // gCO2eq/kWh
    pub abiotic_factor: f32,        // kgSbeq/kWh
    pub primary_energy_factor: f32, // kJ/kWh
}

pub struct Device {
    pub name: String,
    pub components: Components,
    pub system: System,
    pub power_model: PowerModel,
}

impl Device {
    /*
    pub fn new(name: String, components: Components ) -> Device {
        Device { name, components }
    }
    */
    // Calculates the environmental impact based on average consumption.
    pub fn calculate_metrics(&mut self, duration_hours: f32, avg_powers: f32) -> EnergyStats {
        let energy_wh = avg_powers * duration_hours;
        let energy_kwh = energy_wh / 1000.0;
        let ghg_emissions = energy_kwh * self.power_model.emission_factor;
        let abiotic_resources = energy_kwh * self.power_model.abiotic_factor;
        let primary_energy = energy_kwh * self.power_model.primary_energy_factor;
        let average_power = avg_powers;
        EnergyStats {
            energy_wh,
            ghg_emissions,
            abiotic_resources,
            primary_energy,
            average_power,
        }
    }

    pub fn display_metrics(&mut self, duration_hours: f32, avg_powers: f32) {
        let stats = self.calculate_metrics(duration_hours, avg_powers);
        println!("🔋 Energy Consumption: {:.2} Wh", stats.energy_wh);
        println!("🌍 GHG Emissions: {:.2} gCO2eq", stats.ghg_emissions);
        println!(
            "🌿 Abiotic Resources: {:.2} kgSbeq",
            stats.abiotic_resources
        );
        println!("🔋 Primary Energy: {:.2} kJ", stats.primary_energy);
        println!("🔌 Average Power Consumption: {:.2} W", stats.average_power);
    }

    /*
        Estimates the power consumption of the Raspberry Pi 5.
        The Pi 5 has a different architecture than the M2, with a more
        linear relationship between CPU load and power consumption.
    */
    pub fn estimate_power_consumption(&mut self) -> f32 {
        // Calculate the power consumption of the Raspberry Pi 5
        // based on the current CPU usage and RAM consumption
        let cpu_usage = self.average_cpu_usage();
        let cpu_temp = self.average_temperature();

        // Temperature affects the Pi's power consumption more significantly
        // than on a Mac, so we add a temperature factor
        let temp_factor = f32::min(1.0, f32::max(0.0, (cpu_temp - 40.0) / 40.0));

        let memory_usage = self.usage_virtual_memory_percentage() / 100.0;
        let memory_power = memory_usage as f32 * self.power_model.ram_power_factor;

        // Calculate the total power consumption taking into account all factors

        let total_power = self.power_model.base_power
            + (self.power_model.max_power - self.power_model.base_power)
                * (
                    0.70 * (cpu_usage / 100.0) +    // CPU is the main factor
            0.15 * memory_power +           // RAM impact
            0.15 * temp_factor
                    // Temperature impact
                );
        total_power
    }

    #[allow(dead_code)]
    pub fn display_power_consumption(&mut self) {
        let power = self.estimate_power_consumption();
        println!("🔋 Estimated Power Consumption: {:.2}W", power);
    }

    // Total RAM + Swap
    pub fn total_virtual_memory(&mut self) -> u64 {
        self.system.refresh_memory();
        let total_virtual_memory = self.system.total_memory() + self.system.total_swap();
        total_virtual_memory
    }

    // Used RAM + Used Swap
    pub fn used_virtual_memory(&mut self) -> u64 {
        self.system.refresh_memory();
        let used_virtual_memory = self.system.used_memory() + self.system.used_swap();
        used_virtual_memory
    }

    // (used_virtual_memory / total_virtual_memory) * 100
    pub fn usage_virtual_memory_percentage(&mut self) -> f64 {
        let total_virtual_memory = self.total_virtual_memory();
        let used_virtual_memory = self.used_virtual_memory();

        if total_virtual_memory > 0 {
            (used_virtual_memory as f64 / total_virtual_memory as f64) * 100.0
        } else {
            0.0
        }
    }

    #[allow(dead_code)]
    pub fn display_virtual_memory_usage(&mut self) {
        let total = self.total_virtual_memory();
        let used = self.used_virtual_memory();
        let percentage = self.usage_virtual_memory_percentage();

        println!("🧠 Total Virtual Memory: {} octets", total);
        println!("🧠 Used Virtual Memory: {} octets", used);
        println!("🧠 Virtual Memory Usage: {:.2}%", percentage);
    }

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

    #[allow(dead_code)]
    pub fn display_average_temperature(&mut self) {
        let t = self.average_temperature();
        if t > 0.0 {
            println!("🌡️ Average Workstation Temperature: {:.2}°C", t);
        } else {
            println!("✋ No temperature sensors detected.");
        }
    }

    // percentage of CPU usage
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

    #[allow(dead_code)]
    pub fn display_average_cpu_usage(&mut self) {
        let u = self.average_cpu_usage();
        if u > 0.0 {
            println!("📶 Average CPU Consumption: {:.2}%", u);
        } else {
            println!("✋ No CPU data available.");
        }
    }

    pub fn monitor(&mut self, duration_seconds: u64) {
        let start_time = std::time::Instant::now();
        let mut measurements: Vec<f32> = Vec::new();

        while start_time.elapsed().as_secs() < duration_seconds {
            let power = self.estimate_power_consumption();
            measurements.push(power);

            let remaining = duration_seconds - start_time.elapsed().as_secs();
            let cpu_temp = self.average_temperature();
            let cpu_usage = self.average_cpu_usage();
            let mem_usage = self.usage_virtual_memory_percentage();

            println!(
                "🔋 Power Consumption: {:.2} W | 🌡️ CPU Temp: {:.2} °C | 📶 Average CPU Consumption: {:.2}% | 🧠 Virtual Memory Usage: {:.2}% | ⏱️ Time remaining: {} s",
                power, cpu_temp, cpu_usage, mem_usage, remaining
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("\n✅ Monitoring complete!\n");
        let avg_power = measurements.iter().sum::<f32>() / measurements.len() as f32;
        let duration_hours = duration_seconds as f32 / 3600.0;

        self.display_metrics(duration_hours, avg_power);
    }

    pub async fn monitor_completion(
        &mut self,
        ollama_url: &str,
        model: &str,
        prompt: &str,
    ) -> Result<(), Error> {

        let start_time = std::time::Instant::now();
        let mut measurements: Vec<f32> = Vec::new();

        // Create a new HTTP client
        let client = reqwest::ClientBuilder::new().build()?;

        // Define the URL and headers
        //let url = "http://host.docker.internal:11434/api/generate";
        let url = format!("{}/api/generate", ollama_url);

        use reqwest::header::{HeaderMap, HeaderValue};
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        // Define the request body
        // create a json string with model and prompt
        let body = format!(r#"{{"model": "{}", "prompt":"{}"}}"#, model, prompt);
        //let body = r#"{"model": "qwen2.5:1.5b", "prompt":"who is Jean-Luc Picard?"}"#;

        // Make a POST request to the URL
        let mut response = client.post(url).headers(headers).body(body).send().await?;

        // Ensure the request was successful
        if !response.status().is_success() {
            response = response.error_for_status()?;
        }

        // Stream the response body chunk by chunk
        while let Some(chunk) = response.chunk().await? {

            let power = self.estimate_power_consumption();
            measurements.push(power);

            //let cpu_temp = self.average_temperature();
            //let cpu_usage = self.average_cpu_usage();
            //let mem_usage = self.usage_virtual_memory_percentage();


            // Convert the chunk to a string
            let chunk_str = String::from_utf8_lossy(&chunk);

            // Parse the JSON
            if let Ok(json) = serde_json::from_str::<Value>(&chunk_str) {
                // Extract the response field
                if let Some(response_text) = json["response"].as_str() {
                    // Print just the response text
                    print!("{}", response_text);
                    // Flush stdout to ensure immediate display
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                }
            }
        }

        println!("\n✅ Monitoring complete!\n");
        let end_time = std::time::Instant::now();
        let duration_seconds = end_time.duration_since(start_time).as_secs();
        let avg_power = measurements.iter().sum::<f32>() / measurements.len() as f32;
        let duration_hours = duration_seconds as f32 / 3600.0;

        self.display_metrics(duration_hours, avg_power);

        println!(); // Add a newline at the end
        Ok(())
    }
}
