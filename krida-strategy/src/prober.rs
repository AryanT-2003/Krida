use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct Prober;

impl Strategy for Prober {
    fn name(&self) -> &'static str {
        "Prober"
    }

    fn id(&self) -> usize {
        Registry::Prober as usize
    }

    fn decide(&self, _my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        let turn: usize = their_current_history.len();
        match turn {
            0 => Move::Defect,
            1 | 2 => Move::Cooperate,
            _ => {
                if their_current_history[1] == Move::Cooperate
                    && their_current_history[2] == Move::Cooperate
                {
                    Move::Defect
                } else {
                    *their_current_history.last().unwrap_or(&Move::Cooperate)
                }
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
