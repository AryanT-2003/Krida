use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct TwoTitsForTat;

impl Strategy for TwoTitsForTat {
    fn name(&self) -> &'static str {
        "TwoTitsForTat"
    }

    fn id(&self) -> usize {
        Registry::TwoTitsForTat as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        let start = their_current_history.len().saturating_sub(2);
        let last_moves = &their_current_history[start..];

        // Check if any of those recent moves were a Defect
        if last_moves.contains(&Move::Defect) {
            Move::Defect
        } else {
            Move::Cooperate
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
