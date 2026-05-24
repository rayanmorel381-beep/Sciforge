use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static OXYGEN: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("O2", GasFamily::Atmosphere));
