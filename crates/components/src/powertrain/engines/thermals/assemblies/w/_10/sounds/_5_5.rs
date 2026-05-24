use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::LuxuryHum) }
pub fn howl(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W10Howl) }
pub fn race(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::Race) }
pub fn turbo_twin(d: &Displacement) -> EngineSound { turbo(d, ExhaustVoice::W10Howl, true) }
