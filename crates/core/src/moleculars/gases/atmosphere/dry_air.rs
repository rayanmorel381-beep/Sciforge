use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static DRY_AIR: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("AIR", GasFamily::Atmosphere));
