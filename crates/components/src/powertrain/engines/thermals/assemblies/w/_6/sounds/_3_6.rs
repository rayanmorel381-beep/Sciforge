use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::LuxuryHum) }
pub fn sport(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W6Sport) }
pub fn race(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::Race) }
pub fn turbo_twin(d: &Displacement) -> EngineSound { turbo(d, ExhaustVoice::W6Burble, true) }
