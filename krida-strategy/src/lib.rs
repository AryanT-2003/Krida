pub(crate) use krida_core::game::Move;
pub(crate) use krida_core::population::Strategy;

mod always_cooperate;
mod always_defect;
mod bully;
mod grudger;
mod handshake;
mod joss;
mod pavlov;
mod prober;
mod random;
pub mod registry;
mod soft_grudger;
mod tit_for_tat;
mod tit_for_two_tats;
mod two_tits_for_tat;
