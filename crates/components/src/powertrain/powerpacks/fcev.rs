use crate::powertrain::engines::electrics::BatteryPack;

use super::bev::{self, BevPowerpack, BevSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuelCellMembrane {
    Pem,
    SolidOxide,
    Alkaline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HydrogenStorage {
    Compressed350Bar,
    Compressed700Bar,
    Liquid,
    MetalHydride,
}

#[derive(Debug, Clone)]
pub struct FuelCellStack {
    pub membrane: FuelCellMembrane,
    pub cell_count: u16,
    pub continuous_power_kw: f64,
    pub peak_power_kw: f64,
    pub stack_voltage_v: f64,
    pub efficiency: f64,
    pub mass_kg: f64,
}

impl FuelCellStack {
    pub fn pem(cell_count: u16, continuous_power_kw: f64) -> Self {
        let stack_voltage_v = cell_count as f64 * 0.65;
        let mass_kg = continuous_power_kw * 1.6 + 18.0;
        Self {
            membrane: FuelCellMembrane::Pem,
            cell_count,
            continuous_power_kw,
            peak_power_kw: continuous_power_kw * 1.15,
            stack_voltage_v,
            efficiency: 0.55,
            mass_kg: (mass_kg * 100.0).round() / 100.0,
        }
    }

    pub fn solid_oxide(cell_count: u16, continuous_power_kw: f64) -> Self {
        let stack_voltage_v = cell_count as f64 * 0.85;
        let mass_kg = continuous_power_kw * 2.4 + 35.0;
        Self {
            membrane: FuelCellMembrane::SolidOxide,
            cell_count,
            continuous_power_kw,
            peak_power_kw: continuous_power_kw * 1.05,
            stack_voltage_v,
            efficiency: 0.60,
            mass_kg: (mass_kg * 100.0).round() / 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HydrogenTank {
    pub storage: HydrogenStorage,
    pub usable_kg: f64,
    pub pressure_bar: f64,
    pub volume_l: f64,
    pub mass_kg: f64,
}

impl HydrogenTank {
    pub fn new(storage: HydrogenStorage, usable_kg: f64) -> Self {
        let (pressure_bar, density_kg_per_l, shell_factor) = match storage {
            HydrogenStorage::Compressed350Bar => (350.0, 0.024, 18.0),
            HydrogenStorage::Compressed700Bar => (700.0, 0.040, 22.0),
            HydrogenStorage::Liquid => (4.0, 0.071, 30.0),
            HydrogenStorage::MetalHydride => (30.0, 0.060, 65.0),
        };
        let volume_l = usable_kg / density_kg_per_l;
        let mass_kg = usable_kg * shell_factor + 12.0;
        Self {
            storage,
            usable_kg,
            pressure_bar,
            volume_l: (volume_l * 100.0).round() / 100.0,
            mass_kg: (mass_kg * 100.0).round() / 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FcevSpec {
    pub system_voltage_v: f64,
    pub stack: FuelCellStack,
    pub hydrogen_tank: HydrogenTank,
    pub buffer_battery: BatteryPack,
    pub traction: BevSpec,
}

#[derive(Debug, Clone)]
pub struct FcevPowerpack {
    pub system_voltage_v: f64,
    pub stack: FuelCellStack,
    pub hydrogen_tank: HydrogenTank,
    pub buffer_battery: BatteryPack,
    pub traction: BevPowerpack,
    pub electric_peak_power_kw: f64,
    pub electric_peak_torque_nm: f64,
    pub continuous_power_kw: f64,
    pub usable_battery_kwh: f64,
    pub total_mass_kg: f64,
}

pub fn assemble(spec: &FcevSpec) -> FcevPowerpack {
    let traction = bev::assemble(&spec.traction);
    let usable_battery_kwh = spec.buffer_battery.capacity_kwh * 0.80;
    let total_mass_kg = spec.stack.mass_kg
        + spec.hydrogen_tank.mass_kg
        + spec.buffer_battery.mass_kg
        + traction.total_mass_kg;
    FcevPowerpack {
        system_voltage_v: spec.system_voltage_v,
        stack: spec.stack.clone(),
        hydrogen_tank: spec.hydrogen_tank.clone(),
        buffer_battery: spec.buffer_battery.clone(),
        electric_peak_power_kw: traction.peak_power_kw,
        electric_peak_torque_nm: traction.peak_torque_nm,
        continuous_power_kw: spec.stack.continuous_power_kw,
        traction,
        usable_battery_kwh: (usable_battery_kwh * 100.0).round() / 100.0,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}
