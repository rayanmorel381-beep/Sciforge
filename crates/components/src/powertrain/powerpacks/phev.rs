use crate::powertrain::engines::electrics::{
    BatteryPack, EMotor, Inverter,
};

use super::driveline::{self, Driveline, DrivelineSpec};
use super::hev::HevPosition;
use super::ice::IcePowerpack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargePortStandard {
    Type1,
    Type2,
    Ccs1,
    Ccs2,
    Chademo,
    GbT,
    Nacs,
}

#[derive(Debug, Clone)]
pub struct PhevSpec {
    pub position: HevPosition,
    pub system_voltage_v: f64,
    pub ice: IcePowerpack,
    pub motor: EMotor,
    pub battery: BatteryPack,
    pub inverter: Inverter,
    pub driveline: DrivelineSpec,
    pub ac_charge_port: ChargePortStandard,
    pub dc_charge_port: Option<ChargePortStandard>,
    pub max_ac_charge_kw: f64,
    pub max_dc_charge_kw: f64,
    pub ev_range_km: f64,
}

#[derive(Debug, Clone)]
pub struct PhevPowerpack {
    pub position: HevPosition,
    pub system_voltage_v: f64,
    pub ice: IcePowerpack,
    pub motor: EMotor,
    pub battery: BatteryPack,
    pub inverter: Inverter,
    pub driveline: Driveline,
    pub ac_charge_port: ChargePortStandard,
    pub dc_charge_port: Option<ChargePortStandard>,
    pub max_ac_charge_kw: f64,
    pub max_dc_charge_kw: f64,
    pub ev_range_km: f64,
    pub combined_peak_power_kw: f64,
    pub combined_peak_torque_nm: f64,
    pub usable_battery_kwh: f64,
    pub total_mass_kg: f64,
}

pub fn assemble(spec: &PhevSpec) -> PhevPowerpack {
    let driveline = driveline::assemble(&spec.driveline);
    let mut ice = spec.ice.clone();
    ice.driveline = Some(driveline.clone());
    let combined_peak_power_kw = ice.peak_power_kw() + spec.motor.peak_power_kw;
    let combined_peak_torque_nm = ice.max_input_torque_nm() + spec.motor.peak_torque_nm;
    let usable_battery_kwh = spec.battery.capacity_kwh * 0.85;
    let onboard_charger_mass = (spec.max_ac_charge_kw / 3.7) * 4.0;
    let total_mass_kg = ice.total_mass_kg()
        + spec.motor.mass_kg
        + spec.battery.mass_kg
        + spec.inverter.mass_kg
        + onboard_charger_mass;
    PhevPowerpack {
        position: spec.position,
        system_voltage_v: spec.system_voltage_v,
        ice,
        motor: spec.motor.clone(),
        battery: spec.battery.clone(),
        inverter: spec.inverter.clone(),
        driveline,
        ac_charge_port: spec.ac_charge_port,
        dc_charge_port: spec.dc_charge_port,
        max_ac_charge_kw: spec.max_ac_charge_kw,
        max_dc_charge_kw: spec.max_dc_charge_kw,
        ev_range_km: spec.ev_range_km,
        combined_peak_power_kw,
        combined_peak_torque_nm,
        usable_battery_kwh: (usable_battery_kwh * 100.0).round() / 100.0,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}
