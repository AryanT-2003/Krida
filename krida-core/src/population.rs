use crate::game::Move;

pub(crate) struct Player {
    pub(crate) uuid: usize, //uuid collision concern in GameState initiation
    pub(crate) strategy: Box<dyn Strategy>,
}

impl Player {
    pub(crate) fn new(uuid: usize, strategy: Box<dyn Strategy>) -> Self {
        Self { uuid, strategy }
    }

    pub(crate) fn make_move(
        &self,
        my_current_history: &[Move],
        their_current_history: &[Move],
    ) -> Move {
        self.strategy
            .decide(my_current_history, their_current_history)
    }
    pub(crate) fn get_strategy_id(&self) -> usize {
        self.strategy.id()
    }
}

pub trait Strategy: Send + Sync {
    fn name(&self) -> &'static str;

    fn id(&self) -> usize;

    fn decide(&self, my_current_history: &[Move], their_current_history: &[Move]) -> Move;

    fn clone_box(&self) -> Box<dyn Strategy>;
}

impl Clone for Box<dyn Strategy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
