use crate::{Move, Strategy, registry::Registry};

#[derive(Clone)]
pub struct Random;

impl Strategy for Random {
    fn name(&self) -> &'static str {
        "Random"
    }

    fn id(&self) -> usize {
        Registry::Random as usize
    }

    fn decide(&self, _my_current_history: &[Move], _their_current_history: &[Move]) -> Move {
        if rand::random::<bool>() {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
