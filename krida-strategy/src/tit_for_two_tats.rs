use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct TitForTwoTats;

impl Strategy for TitForTwoTats {
    fn name(&self) -> &'static str {
        "TitForTwoTats"
    }

    fn id(&self) -> usize {
        Registry::TitForTwoTats as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        if their_current_history.ends_with(&[Move::Defect, Move::Defect]) {
            Move::Defect
        } else {
            Move::Cooperate
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
