pub mod battery;
pub mod cooling;
pub mod e_motor;
pub mod inverter;

pub use battery::{BatteryPack, CellChemistry, CellFormat};
pub use cooling::{CoolantType, CoolingLoop};
pub use e_motor::{EMotor, MotorTopology};
pub use inverter::{Inverter, SwitchTechnology};

#[derive(Debug, Clone)]
pub struct ElectricEngine {
    pub motor: EMotor,
    pub inverter: Inverter,
    pub battery: BatteryPack,
    pub cooling: CoolingLoop,
}

impl Default for ElectricEngine {
    fn default() -> Self {
        Self::pmsm_nmc(150.0, 400.0, 96, 46, 4.2, 3.6)
    }
}

impl ElectricEngine {
    pub fn assemble(motor: EMotor, battery: BatteryPack) -> Self {
        let inverter = Inverter::sic(motor.peak_power_kw, battery.nominal_voltage_v);
        let losses_kw = motor.peak_losses_kw() + inverter.peak_losses_kw;
        let cooling = CoolingLoop::for_losses(losses_kw);
        Self { motor, inverter, battery, cooling }
    }

    pub fn pmsm_nmc(
        peak_power_kw: f64,
        nominal_voltage_v: f64,
        series: u32,
        parallel: u32,
        cell_ah: f64,
        cell_v: f64,
    ) -> Self {
        let battery = BatteryPack::nmc(CellFormat::Cylindrical2170, series, parallel, cell_ah, cell_v);
        let motor = EMotor::pmsm(peak_power_kw, battery.nominal_voltage_v.max(nominal_voltage_v));
        Self::assemble(motor, battery)
    }

    pub fn induction_lfp(
        peak_power_kw: f64,
        series: u32,
        parallel: u32,
        cell_ah: f64,
        cell_v: f64,
    ) -> Self {
        let battery = BatteryPack::lfp(CellFormat::Prismatic, series, parallel, cell_ah, cell_v);
        let motor = EMotor::induction(peak_power_kw, battery.nominal_voltage_v);
        Self::assemble(motor, battery)
    }

    pub fn axial_flux_solid_state(
        peak_power_kw: f64,
        series: u32,
        parallel: u32,
        cell_ah: f64,
        cell_v: f64,
    ) -> Self {
        let battery = BatteryPack::solid_state(CellFormat::Pouch, series, parallel, cell_ah, cell_v);
        let motor = EMotor::axial_flux(peak_power_kw, battery.nominal_voltage_v);
        let inverter = Inverter::gan(motor.peak_power_kw, battery.nominal_voltage_v);
        let losses_kw = motor.peak_losses_kw() + inverter.peak_losses_kw;
        let cooling = CoolingLoop::dielectric_oil(losses_kw * 1.25);
        Self { motor, inverter, battery, cooling }
    }

    pub fn total_mass_kg(&self) -> f64 {
        self.motor.mass_kg + self.inverter.mass_kg + self.battery.mass_kg + self.cooling.mass_kg
    }
}
