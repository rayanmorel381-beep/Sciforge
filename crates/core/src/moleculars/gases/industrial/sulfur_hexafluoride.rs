use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static SULFUR_HEXAFLUORIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("SF6", GasFamily::Industrial));
