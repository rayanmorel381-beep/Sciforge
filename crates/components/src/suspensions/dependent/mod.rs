pub mod leaf_spring;
pub mod live_axle;
pub mod torsion_beam;

pub use leaf_spring::{LeafSpring, LeafSpringLayout};
pub use live_axle::{DependentLiveAxle, LiveAxleLocating};
pub use torsion_beam::{TorsionBeam, TorsionBeamProfile, TorsionBushingMaterial};

#[derive(Debug, Clone)]
pub enum Dependent {
    LiveAxle(DependentLiveAxle),
    TorsionBeam(TorsionBeam),
    LeafSpring(LeafSpring),
}
