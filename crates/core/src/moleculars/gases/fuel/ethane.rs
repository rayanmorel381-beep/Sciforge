use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static ETHANE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("C2H6", GasFamily::Fuel));
