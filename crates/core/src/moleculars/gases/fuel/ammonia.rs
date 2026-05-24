use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static AMMONIA: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("NH3", GasFamily::Fuel));
