use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct Pavlov;

impl Strategy for Pavlov {
    fn name(&self) -> &'static str {
        "Pavlov"
    }

    fn id(&self) -> usize {
        Registry::Pavlov as usize
    }

    fn decide(&self, my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        match (my_current_history.last(), their_current_history.last()) {
            (None, _) => Move::Cooperate, // Start with Cooperation
            (Some(my_last), Some(their_last)) => {
                if my_last == their_last {
                    // Both C (Reward) or Both D (Punishment) -> Pavlov stays/cooperates
                    // In game theory logic: if we agreed, stay. If we both defected, shift to C.
                    Move::Cooperate
                } else {
                    // One defected, one cooperated -> Shift move
                    Move::Defect
                }
            }
            _ => Move::Cooperate,
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
