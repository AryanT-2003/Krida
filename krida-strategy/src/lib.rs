pub(crate) use krida_core::{Move, Strategy};

pub struct TitForTat {
    pub id: usize,
}

impl Strategy for TitForTat {
    fn id(&self) -> usize {
        self.id
    }
    fn name(&self) -> &str {
        "Tit-for-Tat"
    }
    fn decide(&self, my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        match their_current_history.last() {
            None => Move::Cooperate,
            Some(their_last_move) => *their_last_move,
        }
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(TitForTat { id: self.id })
    }
}

pub struct Grudger {
    pub id: usize,
}

impl Strategy for Grudger {
    fn id(&self) -> usize {
        self.id
    }
    fn name(&self) -> &str {
        "Grudger"
    }
    fn decide(&self, my_current_history: &[Move], their_current_history: &[Move]) -> Move {
        let has_been_betrayed = their_current_history.contains(&Move::Defect);
        if has_been_betrayed {
            Move::Defect
        } else {
            Move::Cooperate
        }
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(Grudger { id: self.id })
    }
}
