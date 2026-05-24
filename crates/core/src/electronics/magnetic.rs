use crate::moleculars::Material;
use sciforge_hub::prelude::constants::MU_0;
use sciforge_hub::prelude::constants::physics::electrodynamics::magnetic;
use sciforge_hub::prelude::physics::electronics::devices as sf_dev;
use std::f64::consts::PI;

impl Material {
    pub fn relative_permeability(&self) -> Option<f64> {
        magnetic::by_formula(self.formula).map(|m| m.relative_permeability)
    }

    pub fn permeability_h_per_m(&self) -> Option<f64> {
        self.relative_permeability().map(|mu_r| mu_r * MU_0)
    }

    pub fn saturation_flux_density_t(&self) -> Option<f64> {
        magnetic::by_formula(self.formula).map(|m| m.saturation_flux_density_t)
    }

    pub fn coercivity_a_per_m(&self) -> Option<f64> {
        magnetic::by_formula(self.formula).map(|m| m.coercivity_a_per_m)
    }

    pub fn curie_temperature_k(&self) -> Option<f64> {
        magnetic::by_formula(self.formula)
            .filter(|m| m.curie_temperature_k > 0.0)
            .map(|m| m.curie_temperature_k)
    }

    pub fn magnetic_kind(&self) -> Option<&'static str> {
        magnetic::by_formula(self.formula).map(|m| m.kind)
    }

    pub fn is_ferromagnetic(&self) -> bool {
        magnetic::by_formula(self.formula).is_some_and(|m| m.kind == "ferro" || m.kind == "hard")
    }

    pub fn is_above_curie(&self, temperature_k: f64) -> Option<bool> {
        self.curie_temperature_k().map(|tc| temperature_k > tc)
    }

    pub fn skin_depth_with_permeability_m(&self, frequency_hz: f64) -> Option<f64> {
        let rho = self.electrical_resistivity_ohm_m()?;
        let mu_r = self.relative_permeability().unwrap_or(1.0);
        Some((rho / (PI * frequency_hz * mu_r * MU_0)).sqrt())
    }

    pub fn solenoid_inductance_h(&self, n_turns: f64, area_m2: f64, length_m: f64) -> Option<f64> {
        self.relative_permeability()
            .map(|mu_r| sf_dev::solenoid_inductance(mu_r, n_turns, area_m2, length_m))
    }
}
