use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static OZONE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("O3", GasFamily::Atmosphere));
