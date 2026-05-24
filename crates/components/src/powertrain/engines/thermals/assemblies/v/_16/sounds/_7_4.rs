use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn classic(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Muffled)
}

pub fn v16_symphony(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V16Symphony)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::V16Symphony, 2)
}
