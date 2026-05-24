use super::{natural, turbo, supercharged, Displacement, EngineSound, ExhaustVoice};

pub fn stock(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Muffled)
}

pub fn sport(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Sporty)
}

pub fn aggressive(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::Aggressive)
}

pub fn tri_y(d: &Displacement) -> EngineSound {
    natural(d, ExhaustVoice::TriYBurble)
}

pub fn turbo_single(d: &Displacement) -> EngineSound {
    turbo(d, ExhaustVoice::Sporty, false)
}

pub fn supercharged_whine(d: &Displacement) -> EngineSound {
    supercharged(d, ExhaustVoice::Sporty, false)
}
