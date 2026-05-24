use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::LuxuryHum) }
pub fn howl(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W10Howl) }
pub fn sport(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::W10Sport) }
pub fn turbo_twin(d: &Displacement) -> EngineSound { turbo(d, ExhaustVoice::W10Howl, true) }
pub fn supercharged_whine(d: &Displacement) -> EngineSound { supercharged(d, ExhaustVoice::W10Sport, false) }
