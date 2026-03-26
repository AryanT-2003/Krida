use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct AlwaysDefect;

impl Strategy for AlwaysDefect {
    fn name(&self) -> &'static str {
        "AlwaysDefect"
    }

    fn id(&self) -> usize {
        Registry::AlwaysDefect as usize
    }

    fn decide(&self, _my_current_history: &[Move], _their_current_history: &[Move]) -> Move {
        Move::Defect
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
