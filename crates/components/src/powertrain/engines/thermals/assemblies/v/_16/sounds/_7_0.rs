use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn v16_symphony(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V16Symphony)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::V16Symphony, 2)
}

pub fn supercharged_howl(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::V16Symphony, true)
}
