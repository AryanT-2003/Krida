use crate::population::Player;
use crate::simulation::Payoff;
use rand::RngExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Cooperate,
    Defect,
}

pub(crate) struct PlayerGameStats {
    uuid: usize,
    strategy_id: usize,
    pub(crate) total_score: u32,
    pub(crate) total_coop_count: usize,
}
impl PlayerGameStats {
    fn new(uuid: usize, strategy_id: usize, total_score: u32, total_coop_count: usize) -> Self {
        Self {
            uuid,
            strategy_id,
            total_score,
            total_coop_count,
        }
    }
}

pub(crate) struct GameSummary {
    pub(crate) player: PlayerGameStats,
    pub(crate) opponent: PlayerGameStats,
    pub(crate) total_dilemmas: usize,
}

pub(crate) struct Game {
    total_dilemmas: usize,
    player_move_history: Vec<Move>,
    opponent_move_history: Vec<Move>,
    player_score: u32,
    opponent_score: u32,
}
impl Game {
    pub(crate) fn new(min_dilemma: usize, max_dilemma: usize) -> Self {
        let total_dilemmas: usize = rand::rng().random_range(min_dilemma..=max_dilemma);

        Self {
            total_dilemmas,
            player_move_history: Vec::with_capacity(total_dilemmas),
            opponent_move_history: Vec::with_capacity(total_dilemmas),
            player_score: 0,
            opponent_score: 0,
        }
    }

    fn resolve_dilemma(
        &self,
        payoff: &Payoff,
        player_move: Move,
        opponent_move: Move,
    ) -> (u32, u32) {
        match (player_move, opponent_move) {
            (Move::Cooperate, Move::Cooperate) => (payoff.reward, payoff.reward),
            (Move::Defect, Move::Defect) => (payoff.punishment, payoff.punishment),
            (Move::Cooperate, Move::Defect) => (payoff.sucker, payoff.temptation),
            (Move::Defect, Move::Cooperate) => (payoff.temptation, payoff.sucker),
        }
    }

    pub(crate) fn run_game(
        &mut self,
        player: &Player,
        opponent: &Player,
        payoff: &Payoff,
    ) -> GameSummary {
        for _ in 0..self.total_dilemmas {
            // 1. Decide Move
            let player_move =
                player.make_move(&self.player_move_history, &self.opponent_move_history);
            let opponent_move =
                opponent.make_move(&self.opponent_move_history, &self.player_move_history);

            // 2. Score
            let (player_gain, opponent_gain) =
                self.resolve_dilemma(payoff, player_move, opponent_move);
            self.player_score += player_gain;
            self.opponent_score += opponent_gain;

            // 3. Record Move
            self.player_move_history.push(player_move);
            self.opponent_move_history.push(opponent_move);
        }

        // 4. Summarise & Return
        let player_coop_count = self
            .player_move_history
            .iter()
            .filter(|&m| *m == Move::Cooperate)
            .count();
        let opponent_coop_count = self
            .opponent_move_history
            .iter()
            .filter(|&m| *m == Move::Cooperate)
            .count();

        GameSummary {
            player: PlayerGameStats::new(
                player.uuid,
                player.get_strategy_id(),
                self.player_score,
                player_coop_count,
            ),
            opponent: PlayerGameStats::new(
                opponent.uuid,
                opponent.get_strategy_id(),
                self.opponent_score,
                opponent_coop_count,
            ),
            total_dilemmas: self.total_dilemmas,
        }
    }
}
