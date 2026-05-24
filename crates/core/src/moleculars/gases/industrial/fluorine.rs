use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static FLUORINE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("F2", GasFamily::Industrial));
