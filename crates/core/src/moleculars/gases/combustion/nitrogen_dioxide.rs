use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static NITROGEN_DIOXIDE: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("NO2", GasFamily::Combustion));
