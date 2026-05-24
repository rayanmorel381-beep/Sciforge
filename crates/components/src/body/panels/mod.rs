pub mod door;
pub mod fender;
pub mod hood;
pub mod roof;
pub mod trunk;

pub use door::{Door, DoorType};
pub use fender::Fender;
pub use hood::{Hood, PanelMaterial};
pub use roof::{Roof, RoofType};
pub use trunk::{Trunk, TrunkStyle};
