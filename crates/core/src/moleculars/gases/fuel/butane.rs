use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static BUTANE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("C4H10", GasFamily::Fuel));
