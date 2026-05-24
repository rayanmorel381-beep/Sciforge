pub mod bentone;
pub mod ptfe;
pub mod silicone;

pub use bentone::*;
pub use ptfe::*;
pub use silicone::*;

use crate::lubrications::Grease;

pub fn all_synthetic_greases() -> Vec<&'static Grease> {
    vec![
        &bentone::BENTONE_GREASE,
        &silicone::SILICONE_GREASE,
        &ptfe::PTFE_GREASE,
    ]
}
