use crate::moleculars::Gas;
use sciforge_hub::prelude::constants::physics::electronics::paschen;
use sciforge_hub::prelude::physics::electronics::amplifiers as sf_amp;
use sciforge_hub::prelude::physics::electronics::devices as sf_dev;

impl Gas {
    pub fn thermal_voltage_v(&self) -> f64 {
        sf_amp::thermal_voltage(self.temperature_ref_k)
    }

    pub fn thermal_voltage_at_v(&self, temperature_k: f64) -> f64 {
        sf_amp::thermal_voltage(temperature_k)
    }

    pub fn paschen_coefficients(&self) -> Option<(f64, f64, f64)> {
        paschen::by_formula(self.formula).map(|p| (p.a_per_pa_m, p.b_v_per_pa_m, p.gamma))
    }

    pub fn breakdown_voltage_v(&self, gap_m: f64) -> Option<f64> {
        let p = paschen::by_formula(self.formula)?;
        Some(sf_dev::paschen_breakdown(
            self.pressure_ref_pa,
            gap_m,
            p.a_per_pa_m,
            p.b_v_per_pa_m,
            p.gamma,
        ))
    }

    pub fn breakdown_voltage_at_v(&self, pressure_pa: f64, gap_m: f64) -> Option<f64> {
        let p = paschen::by_formula(self.formula)?;
        Some(sf_dev::paschen_breakdown(
            pressure_pa,
            gap_m,
            p.a_per_pa_m,
            p.b_v_per_pa_m,
            p.gamma,
        ))
    }

    pub fn breakdown_field_v_per_m(&self, gap_m: f64) -> Option<f64> {
        self.breakdown_voltage_v(gap_m).map(|v| v / gap_m)
    }
}
