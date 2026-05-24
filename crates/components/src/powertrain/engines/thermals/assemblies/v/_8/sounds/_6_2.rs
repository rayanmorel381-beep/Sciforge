use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn flat_plane_scream(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::FlatPlaneScream)
}

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::FlatPlaneScream, 2)
}

pub fn supercharged_howl(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::FlatPlaneScream, true)
}
