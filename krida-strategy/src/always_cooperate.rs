use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn name(&self) -> &'static str {
        "AlwaysCooperate"
    }

    fn id(&self) -> usize {
        Registry::AlwaysCooperate as usize
    }

    fn decide(&self, _my_current_history: &[Move], _their_current_history: &[Move]) -> Move {
        Move::Cooperate
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
