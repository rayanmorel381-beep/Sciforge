use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn sport(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn w12_symphony(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::W12Symphony)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::W12Symphony, 2)
}
