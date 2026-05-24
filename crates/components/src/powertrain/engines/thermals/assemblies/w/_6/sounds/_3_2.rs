use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::LuxuryHum) }
pub fn vr_burble(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W6Burble) }
pub fn sport(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W6Sport) }
pub fn turbo_twin(d: &Displacement) -> EngineSound { turbo(d, ExhaustVoice::W6Burble, true) }
pub fn supercharged_whine(d: &Displacement) -> EngineSound { supercharged(d, ExhaustVoice::W6Sport, false) }
