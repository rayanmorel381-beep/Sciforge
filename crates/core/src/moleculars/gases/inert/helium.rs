use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static HELIUM: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("He", GasFamily::Inert));
