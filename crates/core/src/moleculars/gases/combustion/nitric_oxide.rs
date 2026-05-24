use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static NITRIC_OXIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("NO", GasFamily::Combustion));
