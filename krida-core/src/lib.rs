use rand::RngExt;
use std::collections::HashMap;

/*
Strategy - Generate moves - consuming: History of prev rounds, Current Score, Round No. and produce - a move (Cooperate OR Defect)

Core - Start a Tournament, with N rounds/games, Scoreboard, Move History. For each game 2 players/strategy will provide their move. Evaluate Game result - Who gain, update score, update history. Check if tournament ends. If yes END If no REPEAT.
*/
struct Player {
    uuid: usize,
    strategy: Box<dyn Strategy>,
}

impl Player {
    fn new(uuid: usize, strategy: Box<dyn Strategy>) -> Self {
        Self { uuid, strategy }
    }

    fn make_move(&self, my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        self.strategy
            .decide(my_current_history, their_current_history)
    }
}

pub trait Strategy {
    fn name(&self) -> &str;
    fn id(&self) -> usize;

    fn decide(&self, my_current_history: &[Move], their_current_history: &[Move]) -> Move;

    fn clone_box(&self) -> Box<dyn Strategy>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Cooperate,
    Defect,
}

struct Payoff {
    sucker: u32,
    punishment: u32,
    reward: u32,
    temptation: u32,
}

impl Payoff {
    fn default() -> Self {
        Self {
            sucker: 0,
            punishment: 1,
            reward: 3,
            temptation: 5,
        }
    }

    fn resolve_dilemma(&self, player_move: &Move, opponent_move: &Move) -> Resolution {
        match (player_move, opponent_move) {
            (Move::Cooperate, Move::Cooperate) => Resolution {
                player_gain: self.reward,
                opponent_gain: self.reward,
            },
            (Move::Cooperate, Move::Defect) => Resolution {
                player_gain: self.sucker,
                opponent_gain: self.temptation,
            },
            (Move::Defect, Move::Cooperate) => Resolution {
                player_gain: self.temptation,
                opponent_gain: self.sucker,
            },
            (Move::Defect, Move::Defect) => Resolution {
                player_gain: self.punishment,
                opponent_gain: self.punishment,
            },
        }
    }
}

#[derive(Default)]
struct Scoreboard {
    player_score: u32,
    opponent_score: u32,
}
impl Scoreboard {
    fn update_scoreboard(&mut self, resolution: &Resolution) {
        self.player_score += resolution.player_gain;
        self.opponent_score += resolution.opponent_gain;
    }
}

struct Resolution {
    player_gain: u32,
    opponent_gain: u32,
}

struct GameState {
    total_dilemmas: usize,
    current_dilemma: usize,
    move_history: HashMap<usize, Vec<Move>>,
    scoreboard: Scoreboard,
}
impl GameState {
    fn new(n: usize, player_id: usize, opponent_id: usize) -> Self {
        let mut move_history = HashMap::new();
        move_history.insert(player_id, Vec::with_capacity(n));
        move_history.insert(opponent_id, Vec::with_capacity(n));
        Self {
            total_dilemmas: n,
            current_dilemma: 0,
            scoreboard: Scoreboard::default(),
            move_history,
        }
    }
    fn get_current_history(&self, self_id: usize, their_id: usize) -> (&[Move], &[Move]) {
        let my_current_history = self
            .move_history
            .get(&self_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        let their_current_history = self
            .move_history
            .get(&their_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        (my_current_history, their_current_history)
    }
    fn apply_resolution(
        &mut self,
        player_id: usize,
        opponent_id: usize,
        player_move: Move,
        opponent_move: Move,
        resolution: Resolution,
    ) {
        if let Some(history) = self.move_history.get_mut(&player_id) {
            history.push(player_move);
        }
        if let Some(history) = self.move_history.get_mut(&opponent_id) {
            history.push(opponent_move);
        }
        self.current_dilemma += 1;

        self.scoreboard.update_scoreboard(&resolution);
    }

    fn check_end_criteria(&self) -> bool {
        self.current_dilemma >= self.total_dilemmas
    }
}

fn run_game(player: &Player, opponent: &Player, payoff: &Payoff) -> GameState {
    let n: usize = rand::rng().random_range(150..=250);
    let mut game_state = GameState::new(n, player.uuid, opponent.uuid);

    loop {
        // Player perspective
        let (self_current_history, their_current_history) =
            game_state.get_current_history(player.uuid, opponent.uuid);
        let player_move: Move = player.make_move(self_current_history, their_current_history);

        // Oppoenent perspective
        let (self_current_history, their_current_history) =
            game_state.get_current_history(opponent.uuid, player.uuid);
        let opponent_move: Move = opponent.make_move(self_current_history, their_current_history);

        let resolution = payoff.resolve_dilemma(&player_move, &opponent_move);

        game_state.apply_resolution(
            player.uuid,
            opponent.uuid,
            player_move,
            opponent_move,
            resolution,
        );

        if game_state.check_end_criteria() {
            break;
        }
    }

    game_state
}
