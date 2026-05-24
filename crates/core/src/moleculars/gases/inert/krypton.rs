use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static KRYPTON: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("Kr", GasFamily::Inert));
