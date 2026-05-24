use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static CHLORINE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("Cl2", GasFamily::Industrial));
