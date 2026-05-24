use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static XENON: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("Xe", GasFamily::Inert));
