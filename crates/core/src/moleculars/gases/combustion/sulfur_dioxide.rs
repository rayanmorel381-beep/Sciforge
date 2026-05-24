use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static SULFUR_DIOXIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("SO2", GasFamily::Combustion));
