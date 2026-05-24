use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Muffled)
}

pub fn sport(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn flat_ten_scream(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::FlatTenScream)
}

pub fn turbo_single(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Sporty, false)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Aggressive, true)
}
