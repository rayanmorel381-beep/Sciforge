use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo};

pub fn stock(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::LuxuryHum) }
pub fn sport(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::VR6Sport) }
pub fn turbo_twin(d: &Displacement) -> EngineSound { turbo(d, ExhaustVoice::VR6Sport, true) }
