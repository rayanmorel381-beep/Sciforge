use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, turbo, supercharged};

pub fn stock(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::LuxuryHum) }
pub fn passat_w8(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W8Burble) }
pub fn sport(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W8Sport) }
pub fn turbo_twin(d: &Displacement) -> EngineSound { turbo(d, ExhaustVoice::W8Burble, true) }
pub fn supercharged_whine(d: &Displacement) -> EngineSound { supercharged(d, ExhaustVoice::W8Sport, false) }
