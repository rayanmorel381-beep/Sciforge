use crate::moleculars::Material;
use sciforge_hub::prelude::physics::solid_mechanics::viscoelasticity as sf_visc;

impl Material {
    pub fn maxwell_relaxation_pa(
        &self,
        stress_initial_pa: f64,
        time_s: f64,
        relaxation_time_s: f64,
    ) -> f64 {
        sf_visc::maxwell_relaxation(stress_initial_pa, time_s, relaxation_time_s)
    }

    pub fn kelvin_voigt_strain(
        &self,
        stress_pa: f64,
        time_s: f64,
        retardation_time_s: f64,
    ) -> f64 {
        sf_visc::kelvin_voigt_creep(stress_pa, self.young_modulus_pa, time_s, retardation_time_s)
    }

    pub fn standard_linear_solid_strain(
        &self,
        stress_pa: f64,
        e2_pa: f64,
        eta_pa_s: f64,
        time_s: f64,
    ) -> f64 {
        sf_visc::standard_linear_solid(stress_pa, self.young_modulus_pa, e2_pa, eta_pa_s, time_s)
    }
}

pub fn wlf_shift_factor(t_k: f64, t_ref_k: f64, c1: f64, c2: f64) -> f64 {
    sf_visc::wlf_shift(t_k, t_ref_k, c1, c2)
}

pub fn prony_relaxation_pa(
    e_inf_pa: f64,
    coefficients_e_tau: &[(f64, f64)],
    time_s: f64,
) -> f64 {
    sf_visc::prony_series(e_inf_pa, coefficients_e_tau, time_s)
}
