use krida_core::simulation::SimulationResult;
use std::env;
use std::fs::File;
use std::io::{self, Write};

fn export_timeseries(
    simulation_result: &SimulationResult,
    dir: &std::path::Path,
    active_strat_ids: &[usize],
) -> io::Result<()> {
    let path = dir.join("simulation_timeseries.csv");
    let mut file = File::create(&path)?;

    // Write Header
    write!(file, "Generation,GlobalCooperation")?;
    for &id in active_strat_ids {
        write!(file, ",Count_Strat_{},Fitness_Strat_{}", id, id)?;
    }
    writeln!(file)?;

    // Write Rows
    for gens in 0..simulation_result.total_generation_run {
        let global_coop = simulation_result
            .global_cooperation_history
            .get(gens)
            .unwrap_or(&0.0);
        write!(file, "{},{:.4}", gens, global_coop)?;

        let empty_usize_map = std::collections::HashMap::new();
        let empty_f64_map = std::collections::HashMap::new();

        let pop_map = &simulation_result
            .evolutionary_history
            .get(gens)
            .unwrap_or(&empty_usize_map);
        let fit_map = &simulation_result
            .strategy_fitness_history
            .get(gens)
            .unwrap_or(&empty_f64_map);

        for &id in active_strat_ids {
            let count = pop_map.get(&id).unwrap_or(&0);
            let fitness = fit_map.get(&id).unwrap_or(&0.0);
            write!(file, ",{},{:.4}", count, fitness)?;
        }
        writeln!(file)?;
    }

    println!("Exported: {:?}", path);
    Ok(())
}

fn export_matchups(
    simulation_result: &SimulationResult,
    dir: &std::path::Path,
    grid_size: usize,
) -> io::Result<()> {
    let path = dir.join("simulation_matchups.csv");
    let mut file = File::create(&path)?;

    // Write Header (Melted format for easy plotting in Python/R/Excel)
    writeln!(
        file,
        "Generation,Strat_A,Strat_B,Total_Score,Total_Dilemma,Avg_Score_Per_Move"
    )?;

    // Write Rows
    for gens in 0..simulation_result.total_generation_run {
        if let Some(matrix) = &simulation_result.matchup_matrix_history.get(gens) {
            for a in 0..grid_size {
                for b in 0..grid_size {
                    let idx = a * grid_size + b;

                    if let Some(&(score, dilemmas)) = matrix.get(idx) {
                        // If these two strategies never played, skip the row entirely!
                        if dilemmas == 0 {
                            continue;
                        }

                        // Calculate the average score cleanly
                        let avg_score = score as f64 / dilemmas as f64;
                        writeln!(
                            file,
                            "{},{},{},{},{},{:.4}",
                            gens, a, b, score, dilemmas, avg_score
                        )?;
                    }
                }
            }
        }
    }

    println!("Exported: {:?}", path);
    Ok(())
}

fn export_summary(simulation_result: &SimulationResult, dir: &std::path::Path) -> io::Result<()> {
    let path = dir.join("simulation_summary.txt");
    let mut file = File::create(&path)?;

    writeln!(file, "=== Simulation Summary ===")?;
    writeln!(
        file,
        "Total Generations: {}",
        simulation_result.total_generation_run
    )?;
    writeln!(file, "Stop Reason: {:?}", simulation_result.stop_reason)?;

    println!("Exported: {:?}", path);
    Ok(())
}

pub(crate) fn export_to_csv(simulation_result: &SimulationResult) -> io::Result<()> {
    // 1. Get the directory of the current executable
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    // Calculate the total number of unique strategies based on the matchup matrix length
    // Capacity = N * N, so N = sqrt(Capacity)
    let grid_size = if let Some(first_matrix) = simulation_result.matchup_matrix_history.first() {
        first_matrix.len().isqrt()
    } else {
        0
    };

    let mut active_strat_ids: Vec<usize> =
        if let Some(first_gen_pop) = simulation_result.evolutionary_history.first() {
            first_gen_pop.keys().copied().collect()
        } else {
            Vec::new()
        };
    active_strat_ids.sort_unstable();

    export_timeseries(simulation_result, exe_dir, &active_strat_ids)?;
    export_matchups(simulation_result, exe_dir, grid_size)?;
    export_summary(simulation_result, exe_dir)?;

    Ok(())
}
