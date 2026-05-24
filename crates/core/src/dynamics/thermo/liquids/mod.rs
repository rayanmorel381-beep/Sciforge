pub mod cavitation;
pub mod thermal;

pub use cavitation::*;
pub use thermal::*;

use crate::moleculars::Liquid;

pub struct LiquidState<'a> {
    pub liquid: &'a Liquid,
    pub pressure_pa: f64,
    pub temperature_k: f64,
}

impl<'a> LiquidState<'a> {
    pub fn new(liquid: &'a Liquid, pressure_pa: f64, temperature_k: f64) -> Self {
        Self { liquid, pressure_pa, temperature_k }
    }

    pub fn is_above_vapor_pressure(&self) -> bool {
        self.pressure_pa > self.liquid.vapor_pressure_pa_ref
    }

    pub fn thermal_power_w(&self, flow_rate_m3_s: f64, delta_t_k: f64) -> f64 {
        self.liquid.density_kg_m3_ref
            * flow_rate_m3_s
            * self.liquid.specific_heat_j_kgk
            * delta_t_k
    }

    pub fn reynolds_number(&self, velocity_m_s: f64, length_m: f64) -> f64 {
        self.liquid.reynolds_number(velocity_m_s, length_m)
    }

    pub fn prandtl_number(&self) -> f64 {
        self.liquid.prandtl_number()
    }

    pub fn speed_of_sound_m_s(&self) -> f64 {
        self.liquid.speed_of_sound_m_s()
    }
}
