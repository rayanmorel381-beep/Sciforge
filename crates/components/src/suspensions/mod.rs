pub mod components;
pub mod dependent;
pub mod independent;
pub mod motorcycle;

pub use components::{AntiRollBar, Damper, DamperType, Spring, SpringType};
pub use dependent::{Dependent, DependentLiveAxle, LeafSpring, LeafSpringLayout, TorsionBeam};
pub use independent::{DoubleWishbone, Independent, MacPherson, MultiLink, TrailingArm};
pub use motorcycle::{MotorcycleSuspension, Swingarm, SwingarmType, TelescopicFork};

#[derive(Debug, Clone)]
pub enum Suspension {
    Independent(Independent),
    Dependent(Dependent),
    Motorcycle(MotorcycleSuspension),
}
