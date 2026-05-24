use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn f1_scream(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::F1Scream)
}

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::F1Scream, true)
}
