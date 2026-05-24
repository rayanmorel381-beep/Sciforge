use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static HYDROGEN: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("H2", GasFamily::Fuel));
