use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn bugatti_howl(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::W16BugattiHowl)
}

pub fn w16_thunder(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::W16Thunder)
}

pub fn veyron_quad_turbo(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::W16BugattiHowl, 4, false)
}

pub fn chiron_quad_sequential(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::W16Thunder, 4, true)
}
