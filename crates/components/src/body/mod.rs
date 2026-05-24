pub mod aerodynamics;
pub mod glazing;
pub mod panels;

pub use aerodynamics::{Diffuser, DiffuserMaterial, Spoiler, SpoilerMaterial, Splitter, SplitterMaterial, Wing, WingMount};
pub use glazing::{GlazingType, RearWindow, SideWindow, Windshield};
pub use panels::{Door, DoorType, Fender, Hood, PanelMaterial, Roof, RoofType, Trunk, TrunkStyle};
