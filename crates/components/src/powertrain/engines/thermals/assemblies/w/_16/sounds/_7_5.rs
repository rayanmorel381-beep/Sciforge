use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn w16_symphony(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::W16Symphony)
}

pub fn quad_turbo(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::W16BugattiHowl, 4, false)
}
