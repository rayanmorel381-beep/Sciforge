use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::electrodynamics::dielectric;
use sciforge_hub::prelude::physics::electronics::devices as sf_dev;

impl Material {
    pub fn relative_permittivity(&self) -> Option<f64> {
        dielectric::by_formula(self.formula).map(|d| d.relative_permittivity)
    }

    pub fn dielectric_strength_v_per_m(&self) -> Option<f64> {
        dielectric::by_formula(self.formula).map(|d| d.dielectric_strength_v_per_m)
    }

    pub fn loss_tangent(&self) -> Option<f64> {
        dielectric::by_formula(self.formula).map(|d| d.loss_tangent)
    }

    pub fn breakdown_voltage_v(&self, thickness_m: f64) -> Option<f64> {
        self.dielectric_strength_v_per_m().map(|e| e * thickness_m)
    }

    pub fn parallel_plate_capacitance_f(&self, area_m2: f64, gap_m: f64) -> Option<f64> {
        self.relative_permittivity()
            .map(|eps_r| sf_dev::parallel_plate_capacitance(eps_r, area_m2, gap_m))
    }

    pub fn coaxial_capacitance_per_m(
        &self,
        inner_radius_m: f64,
        outer_radius_m: f64,
    ) -> Option<f64> {
        self.relative_permittivity()
            .map(|eps_r| sf_dev::coaxial_capacitance_per_m(eps_r, inner_radius_m, outer_radius_m))
    }

    pub fn dissipation_factor_at(&self, frequency_hz: f64, capacitance_f: f64) -> Option<f64> {
        let tan_delta = self.loss_tangent()?;
        Some(tan_delta * 2.0 * std::f64::consts::PI * frequency_hz * capacitance_f)
    }

    pub fn dielectric_kind(&self) -> Option<&'static str> {
        let eps_r = self.relative_permittivity()?;
        Some(match eps_r {
            x if x < 2.0 => "vacuum_like",
            x if x < 5.0 => "low_k",
            x if x < 20.0 => "standard",
            _ => "high_k",
        })
    }
}
