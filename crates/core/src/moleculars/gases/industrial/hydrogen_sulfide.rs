use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static HYDROGEN_SULFIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("H2S", GasFamily::Industrial));
