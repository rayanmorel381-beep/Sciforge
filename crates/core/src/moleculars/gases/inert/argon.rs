use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static ARGON: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("Ar", GasFamily::Inert));
