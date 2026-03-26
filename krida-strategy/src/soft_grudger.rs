use crate::registry::Registry;
use crate::{Move, Strategy};

#[derive(Clone)]
pub struct SoftGrudger;

impl Strategy for SoftGrudger {
    fn name(&self) -> &'static str {
        "SoftGrudger"
    }

    fn id(&self) -> usize {
        Registry::SoftGrudger as usize
    }

    fn decide(&self, my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        let len = their_current_history.len();
        if len == 0 {
            return Move::Cooperate;
        }

        // 1. Look back to see if we are currently in the middle of a 6-turn punishment cycle.
        // We check the last 6 moves of our own history.
        for i in 1..=6 {
            if len >= i {
                let lookback_idx = len - i;
                // If we find a Defect in our recent history, we might be punishing.
                // We need to find the START of the most recent punishment.
                if my_current_history[lookback_idx] == Move::Defect {
                    // Find how many moves ago the FIRST defect of this cycle happened.
                    // A cycle is 4 Defects followed by 2 Cooperates.
                    // This logic checks where we are in that 0..6 index.

                    // Simplified logic: Find the first D that wasn't preceded by a D
                    // within the last 6 turns.
                    let mut start_of_cycle = lookback_idx;
                    while start_of_cycle > 0
                        && my_current_history[start_of_cycle - 1] == Move::Defect
                    {
                        start_of_cycle -= 1;
                    }

                    let progress = len - start_of_cycle;
                    if progress < 4 {
                        return Move::Defect;
                    }
                    if progress < 6 {
                        return Move::Cooperate;
                    }
                    // If progress >= 6, the cycle is over. Continue to check opponent.
                    break;
                }
            }
        }

        // 2. If not currently punishing, check if the opponent just defected.
        if *their_current_history.last().unwrap() == Move::Defect {
            Move::Defect // Start new punishment
        } else {
            Move::Cooperate
        }
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
