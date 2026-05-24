use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static R410A: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("R410A", GasFamily::Refrigerant));
