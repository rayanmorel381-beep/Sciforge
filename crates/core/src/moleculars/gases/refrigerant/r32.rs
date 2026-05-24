use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static R32: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("CH2F2", GasFamily::Refrigerant));
