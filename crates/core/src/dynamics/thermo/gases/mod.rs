pub mod mixing;
pub mod processes;

pub use mixing::*;
pub use processes::*;

use crate::moleculars::Gas;

pub struct GasState<'a> {
    pub gas: &'a Gas,
    pub pressure_pa: f64,
    pub temperature_k: f64,
}

impl<'a> GasState<'a> {
    pub fn new(gas: &'a Gas, pressure_pa: f64, temperature_k: f64) -> Self {
        Self { gas, pressure_pa, temperature_k }
    }

    pub fn density_kg_m3(&self) -> f64 {
        self.gas.density_ideal_kg_m3(self.pressure_pa, self.temperature_k)
    }

    pub fn specific_volume_m3_kg(&self) -> f64 {
        1.0 / self.density_kg_m3()
    }

    pub fn enthalpy_j_kg(&self, t_ref_k: f64) -> f64 {
        self.gas.cp_j_kgk_ref * (self.temperature_k - t_ref_k)
    }

    pub fn internal_energy_j_kg(&self, t_ref_k: f64) -> f64 {
        self.gas.cv_j_kgk_ref * (self.temperature_k - t_ref_k)
    }

    pub fn entropy_j_kgk(&self, p_ref_pa: f64, t_ref_k: f64) -> f64 {
        let r = self.gas.specific_gas_constant_j_kgk();
        self.gas.cp_j_kgk_ref * (self.temperature_k / t_ref_k).ln()
            - r * (self.pressure_pa / p_ref_pa).ln()
    }

    pub fn mach_number(&self, velocity_m_s: f64) -> f64 {
        velocity_m_s / self.gas.speed_of_sound_m_s(self.temperature_k)
    }

    pub fn dynamic_viscosity_pa_s(&self) -> f64 {
        self.gas.dynamic_viscosity_pa_s(self.temperature_k)
    }

    pub fn kinematic_viscosity_m2_s(&self) -> f64 {
        self.dynamic_viscosity_pa_s() / self.density_kg_m3()
    }
}
