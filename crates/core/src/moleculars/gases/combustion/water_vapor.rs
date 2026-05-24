use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static WATER_VAPOR: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("H2O", GasFamily::Combustion));
