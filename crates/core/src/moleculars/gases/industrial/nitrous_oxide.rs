use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static NITROUS_OXIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("N2O", GasFamily::Industrial));
