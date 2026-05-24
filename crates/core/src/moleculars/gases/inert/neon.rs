use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static NEON: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("Ne", GasFamily::Inert));
