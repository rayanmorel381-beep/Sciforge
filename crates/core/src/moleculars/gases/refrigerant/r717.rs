use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static R717: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("NH3", GasFamily::Refrigerant));
