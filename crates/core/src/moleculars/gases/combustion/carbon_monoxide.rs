use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static CARBON_MONOXIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("CO", GasFamily::Combustion));
