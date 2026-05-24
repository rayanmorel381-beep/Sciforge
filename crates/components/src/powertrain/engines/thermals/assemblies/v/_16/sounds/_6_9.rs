use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn luxury_hum(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::LuxuryHum)
}

pub fn v16_thunder(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V16Thunder)
}

pub fn turbo_quad(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::V16Thunder, 4)
}

pub fn supercharged_howl(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::V16Thunder, true)
}
