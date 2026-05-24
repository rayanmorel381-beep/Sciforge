use crate::powertrain::engines::electrics::{
    BatteryPack, EMotor, Inverter,
};

use super::driveline::{self, Driveline, DrivelineSpec};
use super::ice::IcePowerpack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MhevPosition {
    P0Belt,
    P1Crank,
}

#[derive(Debug, Clone)]
pub struct MhevSpec {
    pub position: MhevPosition,
    pub system_voltage_v: f64,
    pub ice: IcePowerpack,
    pub bsg: EMotor,
    pub battery: BatteryPack,
    pub inverter: Inverter,
    pub driveline: DrivelineSpec,
}

#[derive(Debug, Clone)]
pub struct MhevPowerpack {
    pub position: MhevPosition,
    pub system_voltage_v: f64,
    pub ice: IcePowerpack,
    pub bsg: EMotor,
    pub battery: BatteryPack,
    pub inverter: Inverter,
    pub driveline: Driveline,
    pub combined_peak_power_kw: f64,
    pub combined_peak_torque_nm: f64,
    pub total_mass_kg: f64,
}

pub fn assemble(spec: &MhevSpec) -> MhevPowerpack {
    let driveline = driveline::assemble(&spec.driveline);
    let mut ice = spec.ice.clone();
    ice.driveline = Some(driveline.clone());
    let combined_peak_power_kw = ice.peak_power_kw() + spec.bsg.peak_power_kw;
    let combined_peak_torque_nm = ice.max_input_torque_nm() + spec.bsg.peak_torque_nm;
    let total_mass_kg = ice.total_mass_kg()
        + spec.bsg.mass_kg
        + spec.battery.mass_kg
        + spec.inverter.mass_kg;
    MhevPowerpack {
        position: spec.position,
        system_voltage_v: spec.system_voltage_v,
        ice,
        bsg: spec.bsg.clone(),
        battery: spec.battery.clone(),
        inverter: spec.inverter.clone(),
        driveline,
        combined_peak_power_kw,
        combined_peak_torque_nm,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}
