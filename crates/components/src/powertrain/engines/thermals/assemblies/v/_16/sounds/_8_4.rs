use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn v16_thunder(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V16Thunder)
}

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn turbo_quad(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::V16Thunder, 4)
}
