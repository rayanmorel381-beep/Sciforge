use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static METHANE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("CH4", GasFamily::Fuel));
