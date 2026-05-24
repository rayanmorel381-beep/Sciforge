use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn luxury_hum(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn w12_howl(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::W12Howl)
}

pub fn w12_symphony(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::W12Symphony)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::W12Symphony, 2)
}

pub fn supercharged_whine(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::W12Symphony, false)
}
