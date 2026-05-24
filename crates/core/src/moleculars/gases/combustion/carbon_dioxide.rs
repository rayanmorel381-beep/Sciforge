use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static CARBON_DIOXIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("CO2", GasFamily::Combustion));
