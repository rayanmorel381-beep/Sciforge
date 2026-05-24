use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static ETHYLENE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("C2H4", GasFamily::Fuel));
