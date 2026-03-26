use crate::game::{Game, GameSummary};
use crate::population::Player;
use crate::simulation::SimulationConfig;

pub(crate) struct PlayerTournamentReport {
    player_uuid: usize,
    pub(crate) strategy_id: usize,
    pub(crate) total_score: u32,
    pub(crate) total_dilemmas: usize,
    total_coops: usize,
}
impl PlayerTournamentReport {
    fn new(player_id: usize, strategy_id: usize) -> Self {
        Self {
            player_uuid: player_id,
            strategy_id,
            total_score: 0,
            total_dilemmas: 0,
            total_coops: 0,
        }
    }
}

pub(crate) struct TournamentResult {
    // 1. For Proportional Reproduction:
    pub(crate) player_reports: Vec<PlayerTournamentReport>,

    // 2. For Analytics:
    pub(crate) total_tournament_coops: usize,
    pub(crate) total_tournament_dilemmas: usize,
    // 3. (Total_Score, Total_Dilemmas)
    pub(crate) matchup_raw_data: Vec<(u32, usize)>,
}
impl TournamentResult {
    fn new(num_players: usize, grid_size: usize) -> Self {
        let matrix_size: usize = grid_size * grid_size;
        Self {
            player_reports: Vec::with_capacity(num_players),
            total_tournament_coops: 0,
            total_tournament_dilemmas: 0,
            matchup_raw_data: vec![(0, 0); matrix_size],
        }
    }
}

pub struct Tournament<'a> {
    players: &'a [Player],
    config: &'a SimulationConfig,
    num_unique_strategies: usize,
    grid_size: usize,
}
impl<'a> Tournament<'a> {
    pub fn new(
        players: &'a [Player],
        config: &'a SimulationConfig,
        num_unique_strategies: usize,
        grid_size: usize,
    ) -> Self {
        Self {
            players,
            config,
            num_unique_strategies,
            grid_size,
        }
    }

    pub fn run(self) -> TournamentResult {
        let num_players: usize = self.players.len();

        let mut result: TournamentResult = TournamentResult::new(num_players, self.grid_size);

        for i in 0..num_players {
            result.player_reports.push(PlayerTournamentReport::new(
                self.players[i].uuid,
                self.players[i].get_strategy_id(),
            ));
        }

        for i in 0..num_players {
            for j in (i + 1)..num_players {
                let player: &Player = &self.players[i];
                let opponent: &Player = &self.players[j];

                // Run the Game
                let mut game: Game = Game::new(self.config.min_dilemmas, self.config.max_dilemmas);
                let game_summary: GameSummary =
                    game.run_game(player, opponent, &self.config.payoff);

                // Update PLAYER's player_reports
                result.player_reports[i].total_dilemmas += game_summary.total_dilemmas;
                result.player_reports[i].total_coops += game_summary.player.total_coop_count;
                result.player_reports[i].total_score += game_summary.player.total_score;

                // Update OPPONENT's player_reports
                result.player_reports[j].total_dilemmas += game_summary.total_dilemmas;
                result.player_reports[j].total_coops += game_summary.opponent.total_coop_count;
                result.player_reports[j].total_score += game_summary.opponent.total_score;

                // Update tournament_totals
                result.total_tournament_dilemmas += game_summary.total_dilemmas;
                result.total_tournament_coops +=
                    game_summary.player.total_coop_count + game_summary.opponent.total_coop_count;

                // Update matchup matrix
                let strat_i: usize = player.get_strategy_id();
                let strat_j: usize = opponent.get_strategy_id();

                // Calculate 1D indices for both perspectives
                let idx_ij: usize = (strat_i * self.grid_size) + strat_j;
                let idx_ji: usize = (strat_j * self.grid_size) + strat_i;

                // Add Strat I's performance against Strat J
                result.matchup_raw_data[idx_ij].0 += game_summary.player.total_score;
                result.matchup_raw_data[idx_ij].1 += game_summary.total_dilemmas;

                // Add Strat J's performance against Strat I
                result.matchup_raw_data[idx_ji].0 += game_summary.opponent.total_score;
                result.matchup_raw_data[idx_ji].1 += game_summary.total_dilemmas;
            }
        }
        result
    }
}
