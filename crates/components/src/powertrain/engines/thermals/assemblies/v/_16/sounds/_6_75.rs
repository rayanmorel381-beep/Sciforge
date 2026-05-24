use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn luxury_hum(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn v16_symphony(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V16Symphony)
}

pub fn supercharged_whine(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::V16Symphony, false)
}
