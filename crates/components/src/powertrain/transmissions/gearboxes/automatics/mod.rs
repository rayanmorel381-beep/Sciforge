pub mod cvt;
pub mod dsg;
pub mod robots;
pub mod torque_converter;

pub use crate::powertrain::transmissions::assemblies::automatics::{
    AutomaticGearbox, AutomaticKind, TorqueConverterState,
};
