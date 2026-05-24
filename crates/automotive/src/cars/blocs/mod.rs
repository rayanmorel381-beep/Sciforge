pub mod engines;
pub mod gearboxes;

pub use crate::components::powertrain::{
    engines::{
        electrics::{BatteryPack, CellFormat, EMotor, Inverter},
        thermals::assemblies::{bloc::Bloc, engine::Engine, fuel::Fuel},
    },
    powerpacks::{
        driveline::{DrivelineSpec, EngineOrientation},
        hev::{assemble as assemble_hev, HevPosition, HevPowerpack, HevSpec},
        IcePowerpack,
    },
    transmissions::{
        assemblies::{
            automatics::AutomaticGearbox,
            cvts::Cvt,
            manuals::ManualGearbox,
            robotized::{RobotizedManual, with_latency},
            sequentials::SequentialGearbox,
        },
        clutches::{Clutch, DryCutch},
        drivelines::hubs::WheelBoltPattern,
        drivingwheels::{
            alls::{Awd, FullTime},
            front::{Fwd, Longitudinal, Transverse},
            rear::{Independent, Rwd},
            DriveLayout,
        },
    },
};
