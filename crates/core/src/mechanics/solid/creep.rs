use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::creep;
use sciforge_hub::prelude::physics::solid_mechanics::creep as sf_creep;

impl Material {
    pub fn norton_a(&self) -> Option<f64> {
        creep::by_formula(self.formula).map(|c| c.norton_a)
    }

    pub fn norton_n(&self) -> Option<f64> {
        creep::by_formula(self.formula).map(|c| c.norton_n)
    }

    pub fn creep_activation_energy_j_per_mol(&self) -> Option<f64> {
        creep::by_formula(self.formula).map(|c| c.activation_energy_q_j_per_mol)
    }

    pub fn larson_miller_constant(&self) -> Option<f64> {
        creep::by_formula(self.formula).map(|c| c.larson_miller_c)
    }

    pub fn creep_max_service_temp_k(&self) -> Option<f64> {
        creep::by_formula(self.formula).map(|c| c.max_service_temp_k)
    }

    pub fn norton_strain_rate_per_s(&self, stress_pa: f64, temperature_k: f64) -> Option<f64> {
        let c = creep::by_formula(self.formula)?;
        Some(sf_creep::norton_strain_rate(
            c.norton_a,
            stress_pa,
            c.norton_n,
            c.activation_energy_q_j_per_mol,
            temperature_k,
        ))
    }

    pub fn time_to_rupture_arrhenius_s(&self, stress_pa: f64, temperature_k: f64) -> Option<f64> {
        let c = creep::by_formula(self.formula)?;
        Some(sf_creep::time_to_rupture_arrhenius(
            c.norton_a,
            stress_pa,
            c.norton_n,
            c.activation_energy_q_j_per_mol,
            temperature_k,
        ))
    }

    pub fn larson_miller_parameter(&self, temperature_k: f64, time_h: f64) -> Option<f64> {
        let c = creep::by_formula(self.formula)?;
        Some(sf_creep::larson_miller_parameter(
            temperature_k,
            time_h,
            c.larson_miller_c,
        ))
    }

    pub fn time_from_larson_miller_h(&self, lmp: f64, temperature_k: f64) -> Option<f64> {
        let c = creep::by_formula(self.formula)?;
        Some(sf_creep::time_from_larson_miller(
            lmp,
            temperature_k,
            c.larson_miller_c,
        ))
    }

    pub fn monkman_grant_rupture_s(
        &self,
        strain_rate_per_s: f64,
        m: f64,
        c_mg: f64,
    ) -> f64 {
        sf_creep::monkman_grant(strain_rate_per_s, m, c_mg)
    }
}
