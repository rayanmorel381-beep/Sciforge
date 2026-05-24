use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static R744: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("CO2", GasFamily::Refrigerant));
