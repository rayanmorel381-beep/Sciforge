use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Muffled)
}

pub fn sport(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn muscle_rumble(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::MuscleRumble)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::MuscleRumble, 2)
}

pub fn supercharged_roots(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::MuscleRumble, false)
}
