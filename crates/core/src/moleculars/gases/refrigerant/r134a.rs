use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static R134A: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("CH2FCF3", GasFamily::Refrigerant));
