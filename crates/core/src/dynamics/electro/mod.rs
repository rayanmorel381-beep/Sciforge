pub mod antenna;
pub mod fields;
pub mod gas;
pub mod liquid;
pub mod material;
pub mod plasma;
pub mod waves;
pub mod waveguide;

pub use sciforge_hub::prelude::physics::electrodynamics::circuits as raw_circuits;
pub use sciforge_hub::prelude::physics::electrodynamics::fields as raw_fields;
pub use sciforge_hub::prelude::physics::electrodynamics::waves as raw_waves;
