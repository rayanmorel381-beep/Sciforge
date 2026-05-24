use crate::components::powertrain::engines::electrics::{
    battery::{BatteryPack, CellChemistry, CellFormat},
    e_motor::{EMotor, MotorTopology},
};

#[derive(Debug, Clone)]
pub struct ElectricPropulsionKit {
    pub motor_topology: MotorTopology,
    pub motors: Vec<EMotor>,
    pub battery_chemistry: CellChemistry,
    pub battery: BatteryPack,
    pub rotor_diameter_m: f64,
}

pub fn quadrotor() -> ElectricPropulsionKit {
    ElectricPropulsionKit {
        motor_topology: MotorTopology::AxialFlux,
        motors: vec![EMotor::axial_flux(15.0, 89.0); 4],
        battery_chemistry: CellChemistry::Nmc,
        battery: BatteryPack::nmc(CellFormat::Cylindrical4680, 24, 18, 5.0, 3.6),
        rotor_diameter_m: 0.80,
    }
}

pub fn hexarotor() -> ElectricPropulsionKit {
    ElectricPropulsionKit {
        motor_topology: MotorTopology::AxialFlux,
        motors: vec![EMotor::axial_flux(20.0, 89.0); 6],
        battery_chemistry: CellChemistry::Nmc,
        battery: BatteryPack::nmc(CellFormat::Cylindrical4680, 24, 27, 5.0, 3.6),
        rotor_diameter_m: 0.70,
    }
}

pub fn all() -> Vec<ElectricPropulsionKit> {
    vec![quadrotor(), hexarotor()]
}
