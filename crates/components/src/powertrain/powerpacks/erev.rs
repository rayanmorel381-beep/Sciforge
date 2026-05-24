use crate::powertrain::engines::electrics::{
    BatteryPack, EMotor, Inverter,
};
use crate::powertrain::engines::thermals::assemblies::engine::Engine;

use super::bev::{self, BevPowerpack, BevSpec};
use super::ice::engine_mass_kg;
use super::phev::ChargePortStandard;

#[derive(Debug, Clone)]
pub struct ErevSpec {
    pub system_voltage_v: f64,
    pub range_extender_engine: Engine,
    pub generator: EMotor,
    pub generator_inverter: Inverter,
    pub buffer_battery: BatteryPack,
    pub traction: BevSpec,
    pub ac_charge_port: Option<ChargePortStandard>,
    pub dc_charge_port: Option<ChargePortStandard>,
    pub max_ac_charge_kw: f64,
    pub max_dc_charge_kw: f64,
}

#[derive(Debug, Clone)]
pub struct ErevPowerpack {
    pub system_voltage_v: f64,
    pub range_extender_engine: Engine,
    pub generator: EMotor,
    pub generator_inverter: Inverter,
    pub buffer_battery: BatteryPack,
    pub traction: BevPowerpack,
    pub ac_charge_port: Option<ChargePortStandard>,
    pub dc_charge_port: Option<ChargePortStandard>,
    pub max_ac_charge_kw: f64,
    pub max_dc_charge_kw: f64,
    pub electric_peak_power_kw: f64,
    pub electric_peak_torque_nm: f64,
    pub genset_continuous_power_kw: f64,
    pub usable_battery_kwh: f64,
    pub total_mass_kg: f64,
}

pub fn assemble(spec: &ErevSpec) -> ErevPowerpack {
    let traction = bev::assemble(&spec.traction);
    let genset_continuous_power_kw = spec
        .range_extender_engine
        .estimated_power_kw
        .min(spec.generator.peak_power_kw);
    let usable_battery_kwh = spec.buffer_battery.capacity_kwh * 0.85;
    let onboard_charger_mass = (spec.max_ac_charge_kw / 3.7) * 4.0;
    let total_mass_kg = engine_mass_kg(&spec.range_extender_engine)
        + spec.generator.mass_kg
        + spec.generator_inverter.mass_kg
        + spec.buffer_battery.mass_kg
        + traction.total_mass_kg
        + onboard_charger_mass;
    ErevPowerpack {
        system_voltage_v: spec.system_voltage_v,
        range_extender_engine: spec.range_extender_engine.clone(),
        generator: spec.generator.clone(),
        generator_inverter: spec.generator_inverter.clone(),
        buffer_battery: spec.buffer_battery.clone(),
        electric_peak_power_kw: traction.peak_power_kw,
        electric_peak_torque_nm: traction.peak_torque_nm,
        traction,
        ac_charge_port: spec.ac_charge_port,
        dc_charge_port: spec.dc_charge_port,
        max_ac_charge_kw: spec.max_ac_charge_kw,
        max_dc_charge_kw: spec.max_dc_charge_kw,
        genset_continuous_power_kw: (genset_continuous_power_kw * 100.0).round() / 100.0,
        usable_battery_kwh: (usable_battery_kwh * 100.0).round() / 100.0,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}
