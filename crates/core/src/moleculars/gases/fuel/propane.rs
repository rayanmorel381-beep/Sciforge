use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static PROPANE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("C3H8", GasFamily::Fuel));
