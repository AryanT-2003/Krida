use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct Bully;

impl Strategy for Bully {
    fn name(&self) -> &'static str {
        "Bully"
    }

    fn id(&self) -> usize {
        Registry::Bully as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        match their_current_history.last() {
            None => Move::Defect,
            Some(Move::Cooperate) => Move::Defect,
            Some(Move::Defect) => Move::Cooperate,
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
