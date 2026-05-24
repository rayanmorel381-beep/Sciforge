use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn v12_symphony(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V12Symphony)
}

pub fn v12_wail(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V12Wail)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::V12Wail, 2)
}
