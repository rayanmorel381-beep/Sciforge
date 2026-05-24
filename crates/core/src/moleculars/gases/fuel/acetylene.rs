use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static ACETYLENE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("C2H2", GasFamily::Fuel));
