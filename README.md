# GasPi
> 🚧 WIP

![gaspi hunting](chasse-au-gaspi.jpg)

**TODO:**
- [ ] Make a CLI with arguments
- [ ] Create a release

## Energy Consumption and Environmental Impact Monitoring Tools

### Goal

The main objective of these programs is to measure (**estimate**) and analyze the energy consumption of computing devices (specifically a Raspberry Pi 5) to understand their environmental impact. The tools provide real-time monitoring and calculate several key environmental metrics to quantify the ecological footprint of running specific software or workloads.

### Specifications

#### Core Functionality

1. **Energy Consumption Monitoring**
   - Real-time power usage estimation based on system metrics
   - Device-specific calibration for accurate measurements
   - Support for Raspberry Pi 5 hardware

2. **Environmental Impact Calculation**
   - Energy consumption (Wh)
   - Greenhouse gas emissions (gCO₂eq)
   - Abiotic resource depletion (kgSbeq)
   - Primary energy usage (kJ)

3. **System Metrics Tracking**
   - CPU utilization (overall and per-core)
   - Memory usage
   - CPU temperature (Raspberry Pi version)
   - Power consumption estimation based on hardware characteristics

#### Device-Specific Adaptations

##### Raspberry Pi 5
- Adjusted for the Broadcom BCM2712 with four Arm Cortex-A76 cores
- Incorporates CPU temperature as a significant factor
- Base power consumption: 5W
- Maximum power consumption: 15W

##### [to come] MacBook Pro M2
- Considers the asymmetric architecture (performance vs. efficiency cores)
- Calibrated for 32GB RAM configuration
- Base power consumption: 7W
- Maximum power consumption: 35W

### Technical Details

- Written in Rust
- Depends on the `sysinfo` library for system metrics
- Uses environmental conversion factors based on French energy mix:
  - Emission factor: 60.0 gCO₂eq/kWh
  - Abiotic factor: 1.39e-6 kgSbeq/kWh
  - Primary energy factor: 10230 kJ/kWh


## Energy Monitoring Methods in Rust

The program implements two distinct monitoring methods that measure power consumption in different usage scenarios:

### 1. Basic Monitoring Method (`monitor`)

This method performs continuous power monitoring over a specified time period:

- Runs for a user-defined duration in seconds
- Takes power measurements once per second
- Displays real-time metrics including:
  - Power consumption in watts
  - CPU temperature
  - Average CPU usage percentage
  - Virtual memory usage percentage
  - Remaining monitoring time
- After completion, calculates and displays the average power consumption and environmental impact metrics based on the duration

### 2. LLM Inference Monitoring Method (`monitor_completion`)

This specialized method measures power consumption during a Large Language Model (LLM) inference task:

- Connects to an Ollama API endpoint
- Sends a specified model name and prompt for processing
- Streams the response from the LLM while simultaneously measuring power consumption
- Displays the LLM's output text in real-time
- Continues monitoring until the complete response is received
- After completion, calculates and displays the average power consumption and environmental metrics for the duration of the inference task

The second method is particularly valuable for measuring the environmental impact of using specific AI models, allowing for comparison between different models' efficiency and ecological footprint during inference tasks.

> The tools enable users to make informed decisions about the environmental impact of their computing workloads and can help in optimizing software for better energy efficiency and reduced ecological footprint.

## Run the GasPi

With `cargo run`:

**Monitor the device during 10 seconds, default is 5 seconds**:
```bash
cargo run -- --duration 10
```

**Monitor the device during am Ollama generate completion**:
```bash
cargo run -- --llm-name qwen2.5:0.5b \
--ollama-url http://localhost:11434 \
--prompt "Who is Jean-Luc Picard?"
```

The power profile of the device is stored in the `.env` file. 

**Use an alternative `.env` file**:
```bash
cargo run -- --env-file ./pi.env --duration 3

cargo run -- --env-file ./pi.env \
--llm-name qwen2.5:3b \
--ollama-url http://localhost:11434 \
--prompt "Who is James T Kirk?"
```