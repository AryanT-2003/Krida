use crate::reporting::export_to_csv;
use krida_core::population::Strategy;
use krida_core::simulation::Simulation;
use krida_core::simulation::SimulationConfig;
use krida_core::simulation::SimulationResult;
use krida_strategy::registry::{Registry, create_strategy};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::{env, fs, process};

mod reporting;

fn parse_cli_args() -> (String, SimulationConfig) {
    let mut args = env::args().skip(1);

    let command = args.next().unwrap_or_else(|| {
        eprintln!("Usage: <exe> run <file.toml> [flags]");
        process::exit(1);
    });

    if command != "run" {
        eprintln!(
            "Error: Unknown command '{}'. Only 'run' is supported.",
            command
        );
        process::exit(1);
    }

    let toml_file = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing .toml file path.");
        process::exit(1);
    });

    let mut config: SimulationConfig = SimulationConfig::default();

    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            let value_str = args.next().unwrap_or_else(|| {
                eprintln!("Error: Missing value for flag '{}'", arg);
                process::exit(1);
            });

            match arg.as_str() {
                "--max_gens" => config.max_gens = value_str.parse().expect("Invalid max_gen"),
                "--min_dilemmas" => {
                    config.min_dilemmas = value_str.parse().expect("Invalid min_dilemma")
                }
                "--max_dilemmas" => {
                    config.max_dilemmas = value_str.parse().expect("Invalid max_dilemma")
                }
                "--stagnation_window" => {
                    config.stagnation_window = value_str.parse().expect("Invalid window")
                }
                "--payoff_sucker" => {
                    config.payoff.sucker = value_str.parse().expect("Invalid sucker")
                }
                "--payoff_reward" => {
                    config.payoff.reward = value_str.parse().expect("Invalid reward")
                }
                "--payoff_punishment" => {
                    config.payoff.punishment = value_str.parse().expect("Invalid punishment")
                }
                "--payoff_temptation" => {
                    config.payoff.temptation = value_str.parse().expect("Invalid temptation")
                }
                _ => eprintln!("Warning: Unknown flag '{}' ignored.", arg),
            }
        }
    }

    (toml_file, config)
}
fn main() {
    let (toml_path, config) = parse_cli_args();

    let toml_content = fs::read_to_string(&toml_path).unwrap_or_else(|_| {
        eprintln!("Error: Could not read file '{}'", toml_path);
        process::exit(1)
    });

    // We expect the TOML to just be key-value pairs of String to usize
    let parsed_toml: BTreeMap<String, usize> =
        toml::from_str(&toml_content).expect("Invalid TOML format");

    // 3. Validate Strategies against Registry
    let mut selected_strategies: Vec<(Registry, usize)> = Vec::new();
    let mut unknown_strategies: Vec<String> = Vec::new();

    for (name, count) in parsed_toml {
        if let Some(registry_enum) = Registry::check_from_str(&name) {
            selected_strategies.push((registry_enum, count));
        } else {
            unknown_strategies.push(name);
        }
    }

    // 4. Reporting & Confirmation
    let total_players: usize = selected_strategies.iter().map(|(_, count)| count).sum();

    println!("\n=== Simulation Pre-Flight Report ===");
    if total_players == 0 {
        println!("Error: No valid players to simulate. Exiting.");
        return;
    }
    println!("Known Strategies Registered: {}", selected_strategies.len());
    println!("Total Players: {}", total_players);
    println!("Config: {:#?}", config);

    if !unknown_strategies.is_empty() {
        println!(
            "⚠️ Unknown Strategies (are ignored): {:?}",
            unknown_strategies
        );
    }

    print!("\nProceed with generating population? (y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() != "y" {
        println!("Aborted.");
        return;
    }

    // 5. Build the Blueprint
    let mut blueprint: Vec<Box<dyn Strategy>> = Vec::with_capacity(total_players);
    for (registry_enum, count) in selected_strategies {
        let prototype = create_strategy(registry_enum);

        for _ in 0..count {
            blueprint.push(prototype.clone_box());
        }
    }

    // 6. Create Simulation
    println!("\nCreating Simulator");
    let simulator: Simulation = Simulation::create(blueprint, config);

    // 7. Run Simulation
    println!("\nRunning Simulator");
    let result: SimulationResult = simulator.simulate();

    // 8. Export CSV results
    println!("Simulation completed, Saving Results");
    if let Err(e) = export_to_csv(&result) {
        eprintln!("Failed to save CSV files: {}", e);
    } else {
        println!("All results saved successfully next to the executable.");
    }
}
