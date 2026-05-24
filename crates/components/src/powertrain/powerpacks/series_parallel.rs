use crate::powertrain::engines::electrics::{
    BatteryPack, EMotor, Inverter,
};
use crate::powertrain::engines::thermals::assemblies::engine::Engine;

use super::driveline::{self, Driveline, DrivelineSpec};
use super::ice::engine_mass_kg;

#[derive(Debug, Clone)]
pub struct PowerSplitDevice {
    pub sun_teeth: u16,
    pub ring_teeth: u16,
    pub planet_teeth: u16,
    pub planet_count: u8,
    pub mass_kg: f64,
}

impl PowerSplitDevice {
    pub fn new(sun_teeth: u16, ring_teeth: u16, planet_teeth: u16, planet_count: u8) -> Self {
        let module_mm = 2.0;
        let ring_dm = (ring_teeth as f64 * module_mm) / 1000.0;
        let mass_kg = ring_dm.powi(2) * 280.0 + 6.0 + (planet_count as f64) * 0.8;
        Self {
            sun_teeth,
            ring_teeth,
            planet_teeth,
            planet_count,
            mass_kg: (mass_kg * 100.0).round() / 100.0,
        }
    }

    pub fn k_ratio(&self) -> f64 {
        self.sun_teeth as f64 / self.ring_teeth as f64
    }
}

#[derive(Debug, Clone)]
pub struct SeriesParallelSpec {
    pub system_voltage_v: f64,
    pub engine: Engine,
    pub generator_mg1: EMotor,
    pub traction_mg2: EMotor,
    pub battery: BatteryPack,
    pub inverter_mg1: Inverter,
    pub inverter_mg2: Inverter,
    pub power_split: PowerSplitDevice,
    pub driveline: DrivelineSpec,
}

#[derive(Debug, Clone)]
pub struct SeriesParallelPowerpack {
    pub system_voltage_v: f64,
    pub engine: Engine,
    pub generator_mg1: EMotor,
    pub traction_mg2: EMotor,
    pub battery: BatteryPack,
    pub inverter_mg1: Inverter,
    pub inverter_mg2: Inverter,
    pub power_split: PowerSplitDevice,
    pub driveline: Driveline,
    pub combined_peak_power_kw: f64,
    pub combined_peak_torque_nm: f64,
    pub usable_battery_kwh: f64,
    pub total_mass_kg: f64,
}

pub fn assemble(spec: &SeriesParallelSpec) -> SeriesParallelPowerpack {
    let driveline = driveline::assemble(&spec.driveline);
    let engine_kw = spec.engine.estimated_power_kw;
    let combined_peak_power_kw = engine_kw + spec.traction_mg2.peak_power_kw;
    let combined_peak_torque_nm = spec.traction_mg2.peak_torque_nm * (1.0 + spec.power_split.k_ratio());
    let usable_battery_kwh = spec.battery.capacity_kwh * 0.55;
    let total_mass_kg = engine_mass_kg(&spec.engine)
        + spec.generator_mg1.mass_kg
        + spec.traction_mg2.mass_kg
        + spec.battery.mass_kg
        + spec.inverter_mg1.mass_kg
        + spec.inverter_mg2.mass_kg
        + spec.power_split.mass_kg
        + driveline.total_mass_kg;
    SeriesParallelPowerpack {
        system_voltage_v: spec.system_voltage_v,
        engine: spec.engine.clone(),
        generator_mg1: spec.generator_mg1.clone(),
        traction_mg2: spec.traction_mg2.clone(),
        battery: spec.battery.clone(),
        inverter_mg1: spec.inverter_mg1.clone(),
        inverter_mg2: spec.inverter_mg2.clone(),
        power_split: spec.power_split.clone(),
        driveline,
        combined_peak_power_kw,
        combined_peak_torque_nm: (combined_peak_torque_nm * 100.0).round() / 100.0,
        usable_battery_kwh: (usable_battery_kwh * 100.0).round() / 100.0,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}
