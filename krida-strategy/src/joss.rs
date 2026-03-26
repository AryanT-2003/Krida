use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct Joss;

impl Strategy for Joss {
    fn name(&self) -> &'static str {
        "Joss"
    }

    fn id(&self) -> usize {
        Registry::Joss as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        if their_current_history.is_empty() {
            return Move::Cooperate;
        }
        if rand::random_bool(0.10) {
            Move::Defect
        } else {
            *their_current_history.last().unwrap_or(&Move::Cooperate)
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
