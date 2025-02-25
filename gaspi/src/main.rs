use clap::Parser;
use dotenv::from_path;
use std::env;
use std::path::PathBuf;
use sysinfo::{Components, System};

// declare module
mod device;

// use module
use device::Device;
use device::PowerModel;

/// Gaspi - Energy monitoring tool for Raspberry Pi 5
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to environment file
    #[arg(short, long, value_name = "FILE")]
    env_file: Option<PathBuf>,

    // Add the name of the llm model
    #[arg(short, long, value_name = "NAME")]
    llm_name: Option<String>,

    // Add the url of ollama
    #[arg(short, long, value_name = "URL")]
    ollama_url: Option<String>,

    // Add the prompt
    #[arg(short, long, value_name = "PROMPT")]
    prompt: Option<String>,

    // Add the duration in seconds
    #[arg(short, long)]
    duration: Option<u64>,
}

#[tokio::main] // Add this attribute to use async main
async fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Load environment variables from specified file or default .env
    if let Some(env_path) = args.env_file {
        if let Err(e) = from_path(&env_path) {
            eprintln!("Error loading environment file: {}", e);
            // You can choose to exit or continue with defaults
        }
    } else {
        // Try to load from default .env file in current directory
        dotenv::dotenv().ok();
    }

    // Create device with values from environment variables
    let mut device = Device {
        name: env::var("DEVICE_NAME").unwrap_or_else(|_| String::from("Raspberry Pi 5")),
        components: Components::new_with_refreshed_list(),
        system: System::new_all(),
        power_model: PowerModel {
            base_power: env::var("BASE_POWER") // Minimum power in watts (idle)
                .unwrap_or_else(|_| String::from("5.0"))
                .parse::<f32>()
                .unwrap_or(5.0),
            max_power: env::var("MAX_POWER") // Maximum power in watts (full load)
                .unwrap_or_else(|_| String::from("15.0"))
                .parse::<f32>()
                .unwrap_or(15.0),
            ram_power_factor: env::var("RAM_POWER_FACTOR") // Higher impact of RAM on Pi (// The Pi 5's RAM is shared with the GPU, which affects power consumption)
                .unwrap_or_else(|_| String::from("0.25"))
                .parse::<f32>()
                .unwrap_or(0.25),
                /*
                    Environmental factors for France
                    These values remain the same as they depend on the energy mix
                */
            emission_factor: env::var("EMISSION_FACTOR") // gCO2eq/kWh
                .unwrap_or_else(|_| String::from("60.0"))
                .parse::<f32>()
                .unwrap_or(60.0),
            abiotic_factor: env::var("ABIOTIC_FACTOR") // kgSbeq/kWh
                .unwrap_or_else(|_| String::from("1.39e-6"))
                .parse::<f32>()
                .unwrap_or(1.39e-6),
            primary_energy_factor: env::var("PRIMARY_ENERGY_FACTOR") // kJ/kWh
                .unwrap_or_else(|_| String::from("10230.0"))
                .parse::<f32>()
                .unwrap_or(10230.0),
        },
    };

    println!("🖥️ Device name: {}", device.name);

    // test if arg.llm_name is not None
    if let Some(llm_name) = args.llm_name {
        // cargo run -- --llm-name qwen2.5:0.5b --ollama-url http://localhost:11434 --prompt "Who is Jean-Luc Picard?"
        println!("🔋 Monitoring 🦙 Ollama completion:");
        // tests if arg.ollama_url is not None else use default: http://localhost:11434
        let mut url = String::from("http://localhost:11434");
        if let Some(ollama_url) = args.ollama_url {
            url = ollama_url;
        }
        println!("🤖 LLM model name: {}", llm_name);
        println!("🦙 Ollama URL: {}", url);

        // monitoring completion
        if let Err(e) = device
            .monitor_completion(
                &url,
                &llm_name,
                args.prompt.unwrap_or_else(|| String::from("Who is John Doe?")).as_str(),
            )
            .await
        {
            eprintln!("Error: {:?}", e);
        }
    } else {
        // monitoring
        // cargo run -- --duration 10
        let duration = args.duration.unwrap_or(5);
        println!("🔋 Monitoring device for {} seconds:", duration);
        device.monitor(duration);
    }

}
