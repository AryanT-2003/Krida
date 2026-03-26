use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct Grudger;

impl Strategy for Grudger {
    fn name(&self) -> &'static str {
        "Grudger"
    }

    fn id(&self) -> usize {
        Registry::Grudger as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        let has_been_betrayed = their_current_history.contains(&Move::Defect);
        if has_been_betrayed {
            Move::Defect
        } else {
            Move::Cooperate
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
