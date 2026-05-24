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

pub fn race(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Race)
}

pub fn boxer_rumble(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::BoxerRumble)
}

pub fn turbo_single(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Sporty, false)
}

pub fn turbo_twin(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Aggressive, true)
}

pub fn supercharged_whine(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::Sporty, false)
}

pub fn supercharged_howl(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::Aggressive, true)
}
