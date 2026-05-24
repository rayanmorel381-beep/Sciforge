use std::sync::LazyLock;
use crate::moleculars::{Gas, GasFamily};

pub static NITROGEN: LazyLock<Gas> = LazyLock::new(|| Gas::from_lib("N2", GasFamily::Atmosphere));
