use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Muffled)
}

pub fn sport(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn v8_burble(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::V8Burble)
}

pub fn muscle_rumble(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::MuscleRumble)
}

pub fn turbo_single(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Sporty, 1)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Aggressive, 2)
}

pub fn supercharged_whine(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::MuscleRumble, false)
}
