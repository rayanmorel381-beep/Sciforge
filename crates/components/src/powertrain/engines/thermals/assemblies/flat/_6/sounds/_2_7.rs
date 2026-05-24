use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Muffled)
}

pub fn sport(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn flat_six_wail(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::FlatSixWail)
}

pub fn turbo_single(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Sporty, false)
}
