use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct TitForTat;

impl Strategy for TitForTat {
    fn name(&self) -> &'static str {
        "TitForTat"
    }

    fn id(&self) -> usize {
        Registry::TitForTat as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        match their_current_history.last() {
            None => Move::Cooperate,
            Some(their_last_move) => *their_last_move,
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
