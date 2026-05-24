pub mod boundary_layer;
pub mod flow;
pub mod turbulence;
pub mod waves;

pub use sciforge_hub::prelude::physics::fluid_mechanics::boundary_layer as raw_boundary_layer;
pub use sciforge_hub::prelude::physics::fluid_mechanics::flow as raw_flow;
pub use sciforge_hub::prelude::physics::fluid_mechanics::turbulence as raw_turbulence;
pub use sciforge_hub::prelude::physics::fluid_mechanics::waves as raw_waves;
