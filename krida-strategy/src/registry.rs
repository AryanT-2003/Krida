use krida_core::population::Strategy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Registry {
    AlwaysCooperate = 0,
    AlwaysDefect = 1,
    Bully = 2,
    Grudger = 3,
    Handshake = 4,
    Joss = 5,
    Pavlov = 6,
    Prober = 7,
    Random = 8,
    SoftGrudger = 9,
    TitForTat = 10,
    TitForTwoTats = 11,
    TwoTitsForTat = 12,
}

impl Registry {
    pub fn check_from_str(name: &str) -> Option<Self> {
        match name {
            "AlwaysCooperate" => Some(Self::AlwaysCooperate),
            "AlwaysDefect" => Some(Self::AlwaysDefect),
            "Bully" => Some(Self::Bully),
            "Grudger" => Some(Self::Grudger),
            "Handshake" => Some(Self::Handshake),
            "Joss" => Some(Self::Joss),
            "Pavlov" => Some(Self::Pavlov),
            "Prober" => Some(Self::Prober),
            "Random" => Some(Self::Random),
            "SoftGrudger" => Some(Self::SoftGrudger),
            "TitForTat" => Some(Self::TitForTat),
            "TitForTwoTats" => Some(Self::TitForTwoTats),
            "TwoTitsForTat" => Some(Self::TwoTitsForTat),
            _ => None,
        }
    }
}

pub fn create_strategy(registry: Registry) -> Box<dyn Strategy> {
    match registry {
        Registry::AlwaysCooperate => Box::new(super::always_cooperate::AlwaysCooperate),
        Registry::AlwaysDefect => Box::new(super::always_defect::AlwaysDefect),
        Registry::Bully => Box::new(super::bully::Bully),
        Registry::Grudger => Box::new(super::grudger::Grudger),
        Registry::Handshake => Box::new(super::handshake::Handshake),
        Registry::Joss => Box::new(super::joss::Joss),
        Registry::Pavlov => Box::new(super::pavlov::Pavlov),
        Registry::Prober => Box::new(super::prober::Prober),
        Registry::Random => Box::new(super::random::Random),
        Registry::SoftGrudger => Box::new(super::soft_grudger::SoftGrudger),
        Registry::TitForTat => Box::new(super::tit_for_tat::TitForTat),
        Registry::TitForTwoTats => Box::new(super::tit_for_two_tats::TitForTwoTats),
        Registry::TwoTitsForTat => Box::new(super::two_tits_for_tat::TwoTitsForTat),
        _ => unimplemented!("Strategy registered but missing from factory"),
    }
}
