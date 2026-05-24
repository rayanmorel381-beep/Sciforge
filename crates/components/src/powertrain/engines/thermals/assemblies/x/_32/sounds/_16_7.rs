use super::super::Displacement;
use super::{EngineSound, ExhaustVoice, natural, supercharged, turbo};

pub fn stock(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::Muffled) }
pub fn sport(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::Sporty) }
pub fn race(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::Race) }
pub fn x_engine_roar(d: &Displacement) -> EngineSound { natural(d, ExhaustVoice::XEngineRoar) }
pub fn turbo_twin(d: &Displacement) -> EngineSound { turbo(d, ExhaustVoice::XEngineRoar, true) }
pub fn supercharged_standard(d: &Displacement) -> EngineSound { supercharged(d, ExhaustVoice::XEngineRoar, false) }
