use crate::population::{Player, Strategy};
use crate::tournament::{PlayerTournamentReport, Tournament, TournamentResult};
use rand::RngExt;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Payoff {
    pub sucker: u32,
    pub punishment: u32,
    pub reward: u32,
    pub temptation: u32,
}
impl Default for Payoff {
    fn default() -> Self {
        Self {
            sucker: 0,
            punishment: 1,
            reward: 3,
            temptation: 5,
        }
    }
}

#[derive(Debug)]
pub struct SimulationConfig {
    pub payoff: Payoff,
    pub max_gens: usize,
    pub stagnation_window: usize,
    pub min_dilemmas: usize,
    pub max_dilemmas: usize,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            payoff: Payoff::default(),
            max_gens: 1000,
            stagnation_window: 5,
            min_dilemmas: 150,
            max_dilemmas: 250,
        }
    }
}

pub struct Simulation {
    population: Vec<Player>,

    config: SimulationConfig,

    current_gen: usize,

    num_unique_strategies: usize,
    grid_size: usize,

    // Index = Generation, Key = Strategy ID, Value = Count of Players
    evolutionary_history: Vec<HashMap<usize, usize>>,

    // Index = Generation, Key = Strategy ID, Value = Avg Fitness
    strategy_fitness_history: Vec<HashMap<usize, f64>>,

    // total_tournament_coops divided by total_tournament_moves
    global_cooperation_history: Vec<f64>,

    // For Matchup Matrix - 1D Array as a 2D Matrix. Capacity = Total Unique Strategy * Total Unique Strategy. If Strategies are ID sequentially 0, 1, 2, 3 .. then (avg total score divided by total dilemmas) of Strat ID 03 vs Strat ID 06 when total unique strategies are 10 is [3 * 10] + 6 = 36.
    matchup_matrix_history: Vec<Vec<(u32, usize)>>,
}

#[derive(Debug)]
pub enum StopReason {
    MaxGenerationsReached,
    StrategyEquilibrium,
    Stagnation,
}

pub struct SimulationResult {
    pub total_generation_run: usize,
    pub stop_reason: StopReason,
    pub global_cooperation_history: Vec<f64>,
    pub evolutionary_history: Vec<HashMap<usize, usize>>,
    pub strategy_fitness_history: Vec<HashMap<usize, f64>>,
    pub matchup_matrix_history: Vec<Vec<(u32, usize)>>,
}

impl Simulation {
    pub fn create(blueprint: Vec<Box<dyn Strategy>>, config: SimulationConfig) -> Self {
        let capacity = config.max_gens;
        let mut population: Vec<Player> = Vec::with_capacity(blueprint.len());
        let mut unique_strategy = HashSet::new();
        for (uuid, strategy_box) in blueprint.into_iter().enumerate() {
            unique_strategy.insert(strategy_box.id());
            population.push(Player::new(uuid, strategy_box));
        }

        // Find the highest Strategy ID and setting Grid size
        let max_id: usize = unique_strategy.iter().max().copied().unwrap_or(0);
        let grid_size: usize = max_id + 1;

        Self {
            population,
            config,
            current_gen: 0,
            num_unique_strategies: unique_strategy.len(),
            grid_size,
            evolutionary_history: Vec::with_capacity(capacity),
            strategy_fitness_history: Vec::with_capacity(capacity),
            global_cooperation_history: Vec::with_capacity(capacity),
            matchup_matrix_history: Vec::with_capacity(capacity),
        }
    }

    pub fn simulate(mut self) -> SimulationResult {
        let end_reason = loop {
            // --- PHASE 1: THE CURRENT GENERATION ---

            // 1. Record Population
            self.record_population_distribution();

            // 2. Create & Run the Tournament
            let tournament = Tournament::new(
                &self.population,
                &self.config,
                self.num_unique_strategies,
                self.grid_size,
            );
            let tournament_result = tournament.run();

            // 3. Record Analytics
            self.record_tournament_analytics(&tournament_result);

            // --- PHASE 2: THE END CHECK ---

            // 4. Check End Criteria
            if let Some(reason) = self.check_end_simulation() {
                break reason;
            }
            // if self.check_end_simulation() {
            //     break;
            // }

            // --- PHASE 3: THE NEXT GENERATION ---

            // 5. Evolve Population (Fitness Proportionate Selection or Roulette Wheel Selection)
            self.update_population(&tournament_result.player_reports);

            self.current_gen += 1;
        };
        SimulationResult {
            total_generation_run: self.current_gen,
            stop_reason: end_reason,
            global_cooperation_history: self.global_cooperation_history,
            evolutionary_history: self.evolutionary_history,
            strategy_fitness_history: self.strategy_fitness_history,
            matchup_matrix_history: self.matchup_matrix_history,
        }
    }
    fn record_population_distribution(&mut self) {
        let mut current_distribution: HashMap<usize, usize> = HashMap::new();
        for player in &self.population {
            *current_distribution
                .entry(player.get_strategy_id())
                .or_insert(0) += 1;
        }
        self.evolutionary_history.push(current_distribution);
    }
    fn record_tournament_analytics(&mut self, tournament_result: &TournamentResult) {
        // Calculate & Record Global Cooperation Rate
        let coop_rate = if tournament_result.total_tournament_dilemmas > 0 {
            tournament_result.total_tournament_coops as f64
                / tournament_result.total_tournament_dilemmas as f64
        } else {
            0.0
        };
        self.global_cooperation_history.push(coop_rate);

        // Calculate & Record Strategy Fitness
        let mut strat_totals: HashMap<usize, (u32, usize)> = HashMap::new();
        for report in &tournament_result.player_reports {
            let entry = strat_totals.entry(report.strategy_id).or_insert((0, 0));
            entry.0 += report.total_score;
            entry.1 += report.total_dilemmas;
        }

        let mut strat_fitness: HashMap<usize, f64> = HashMap::new();
        for (strat_id, (total_score, total_dilemmas)) in strat_totals {
            let fitness = if total_dilemmas > 0 {
                total_score as f64 / total_dilemmas as f64
            } else {
                0.0
            };
            strat_fitness.insert(strat_id, fitness);
        }
        self.strategy_fitness_history.push(strat_fitness);

        // Record Match Matrix
        self.matchup_matrix_history
            .push(tournament_result.matchup_raw_data.clone());
    }

    fn update_population(&mut self, player_tournament_reports: &[PlayerTournamentReport]) {
        let num_player = self.population.len();

        // Phase 1: Building the Pie (Calculating Fitness)
        let mut fitness_scores: Vec<f64> = Vec::with_capacity(num_player);
        let mut total_population_fitness: f64 = 0.0;

        for player_report in player_tournament_reports {
            // prevent division by zero
            let fitness: f64 = if player_report.total_dilemmas > 0 {
                player_report.total_score as f64 / player_report.total_dilemmas as f64
            } else {
                0.0
            };

            fitness_scores.push(fitness);
            total_population_fitness += fitness;
        }

        let mut next_generation: Vec<Player> = Vec::with_capacity(num_player);

        // Phase 2: Spinning the Wheel (The Loop)
        for new_uuid in 0..num_player {
            // Pick a random float between 0.0 and the Total Pie Size
            let spin = rand::rng().random_range(0.0..total_population_fitness);

            let mut running_sum = 0.0;
            let mut selected_index = 0;

            // Phase 3: Finding the Winner (The Running Sum): Find which player's "slice" the spin landed in
            for (index, &fitness) in fitness_scores.iter().enumerate() {
                running_sum += fitness;
                if running_sum >= spin {
                    selected_index = index;
                    break;
                }
            }

            // 4. Clone the Winner: we go to the OLD population, find the winner, and clone their strategy
            let winning_strategy = self.population[selected_index].strategy.clone_box();

            // 5. Birth the Child (New UUID, cloned strategy): we use the loop index as the new UUID to keep UUIDs 0-99 for simplicity
            let child = Player::new(new_uuid, winning_strategy);
            next_generation.push(child);
        }

        // 6. Death and Replacement: the old self.population is dropped, replaced by the children.
        self.population = next_generation;
    }

    fn check_end_simulation(&self) -> Option<StopReason> {
        // Max Generation Cap
        if self.current_gen >= self.config.max_gens {
            return Some(StopReason::MaxGenerationsReached);
        }

        // Strategy Fixation/Equilibrium
        let current_gen_distribution = self.evolutionary_history.last()?;
        if current_gen_distribution.values().any(|&count| count >= 99) {
            return Some(StopReason::StrategyEquilibrium);
        }

        // Stagnation

        // Look back 5 generations
        let stagnation_window = self.config.stagnation_window;

        // Only check for stagnation if we've actually run for at least 5 generations
        if self.evolutionary_history.len() >= stagnation_window {
            // Slice the last 5 generations from the history vector
            let recent_history =
                &self.evolutionary_history[self.evolutionary_history.len() - stagnation_window..];

            // In Rust, two HashMaps can be directly compared with `==`.
            // It checks if all keys and values are identical.
            // .all() will return true if every historical map matches the current one perfectly.
            let is_stagnant = recent_history
                .iter()
                .all(|dist| dist == current_gen_distribution);

            if is_stagnant {
                return Some(StopReason::Stagnation);
            }
        }
        // If none of the end criteria are met, keep evolving!
        None
    }
}
