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
        motors: vec![EMotor::axial_flux(3.5, 48.0); 4],
        battery_chemistry: CellChemistry::Nmc,
        battery: BatteryPack::nmc(CellFormat::Cylindrical2170, 13, 3, 4.5, 3.6),
        rotor_diameter_m: 0.38,
    }
}

pub fn fixed_wing() -> ElectricPropulsionKit {
    ElectricPropulsionKit {
        motor_topology: MotorTopology::Pmsm,
        motors: vec![EMotor::pmsm(2.0, 48.0)],
        battery_chemistry: CellChemistry::Nmc,
        battery: BatteryPack::nmc(CellFormat::Cylindrical2170, 13, 2, 4.5, 3.6),
        rotor_diameter_m: 0.50,
    }
}

pub fn all() -> Vec<ElectricPropulsionKit> {
    vec![quadrotor(), fixed_wing()]
}
