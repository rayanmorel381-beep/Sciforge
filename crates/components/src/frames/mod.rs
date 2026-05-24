pub mod chassis;
pub mod monocoque;
pub mod motorcycle;

pub use chassis::{BackboneFrame, Chassis, LadderFrame, LadderMaterial, Spaceframe, TubeMaterial};
pub use monocoque::{Monocoque, SkinMaterial, SteelGrade, StressedSkin, Unibody};
pub use motorcycle::{CradleFrame, CradleLayout, MotorcycleFrame, PerimeterFrame, TrellisMaterial, TrellisFrame};

#[derive(Debug, Clone)]
pub enum Frame {
    Monocoque(Monocoque),
    Chassis(Chassis),
    Motorcycle(MotorcycleFrame),
}
