use crate::powertrain::engines::electrics::{
    BatteryPack, EMotor, Inverter,
};

use super::driveline::{self, Driveline, DrivelineSpec};
use super::ice::IcePowerpack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HevPosition {
    P0Belt,
    P1Crank,
    P2BetweenClutchAndGearbox,
    P3GearboxOutput,
    P4RearAxle,
}

#[derive(Debug, Clone)]
pub struct HevSpec {
    pub position: HevPosition,
    pub system_voltage_v: f64,
    pub ice: IcePowerpack,
    pub motor: EMotor,
    pub battery: BatteryPack,
    pub inverter: Inverter,
    pub driveline: DrivelineSpec,
    pub ev_only_speed_kmh: f64,
}

#[derive(Debug, Clone)]
pub struct HevPowerpack {
    pub position: HevPosition,
    pub system_voltage_v: f64,
    pub ice: IcePowerpack,
    pub motor: EMotor,
    pub battery: BatteryPack,
    pub inverter: Inverter,
    pub driveline: Driveline,
    pub ev_only_speed_kmh: f64,
    pub combined_peak_power_kw: f64,
    pub combined_peak_torque_nm: f64,
    pub usable_battery_kwh: f64,
    pub total_mass_kg: f64,
}

pub fn assemble(spec: &HevSpec) -> HevPowerpack {
    let driveline = driveline::assemble(&spec.driveline);
    let mut ice = spec.ice.clone();
    ice.driveline = Some(driveline.clone());
    let combined_peak_power_kw = ice.peak_power_kw() + spec.motor.peak_power_kw;
    let combined_peak_torque_nm = ice.max_input_torque_nm() + spec.motor.peak_torque_nm;
    let usable_battery_kwh = spec.battery.capacity_kwh * 0.6;
    let total_mass_kg = ice.total_mass_kg()
        + spec.motor.mass_kg
        + spec.battery.mass_kg
        + spec.inverter.mass_kg;
    HevPowerpack {
        position: spec.position,
        system_voltage_v: spec.system_voltage_v,
        ice,
        motor: spec.motor.clone(),
        battery: spec.battery.clone(),
        inverter: spec.inverter.clone(),
        driveline,
        ev_only_speed_kmh: spec.ev_only_speed_kmh,
        combined_peak_power_kw,
        combined_peak_torque_nm,
        usable_battery_kwh: (usable_battery_kwh * 100.0).round() / 100.0,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}
