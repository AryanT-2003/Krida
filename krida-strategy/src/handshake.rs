use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct Handshake;

impl Strategy for Handshake {
    fn name(&self) -> &'static str {
        "Handshake"
    }

    fn id(&self) -> usize {
        Registry::Handshake as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        let code = [Move::Cooperate, Move::Defect, Move::Cooperate];
        let turn = their_current_history.len();

        // 1. Play the secret handshake for the first 3 turns
        if turn < code.len() {
            return code[turn];
        }

        // 2. On turn 4, check if the opponent played the same code back to us
        let opponent_sent_code = their_current_history[0..3] == code;

        if opponent_sent_code {
            Move::Cooperate // Allied!
        } else {
            Move::Defect // Intruder detected!
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
