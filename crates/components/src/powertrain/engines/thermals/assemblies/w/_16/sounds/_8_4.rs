use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn w16_thunder(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::W16Thunder)
}

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn quad_sequential(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::W16Thunder, 4, true)
}
