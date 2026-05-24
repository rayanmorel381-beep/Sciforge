use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::optics::thermo_optic;
use sciforge_hub::prelude::physics::optics::devices as sf_dev;

impl Material {
    pub fn dn_dt_per_k(&self) -> Option<f64> {
        thermo_optic::by_formula(self.formula).map(|t| t.dn_dt_per_k)
    }

    pub fn refractive_index_at_temperature(
        &self,
        n_ref: f64,
        temperature_k: f64,
    ) -> Option<f64> {
        let t = thermo_optic::by_formula(self.formula)?;
        Some(sf_dev::n_temperature(n_ref, t.dn_dt_per_k, temperature_k, t.temperature_k))
    }
}
