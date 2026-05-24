use crate::lubrications::Grease;
use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::friction;
use sciforge_hub::prelude::constants::physics::solid_mechanics::wear;
use sciforge_hub::prelude::physics::solid_mechanics::contact as sf_contact;

#[derive(Debug, Clone, Copy)]
pub struct LubricatedPair {
    pub grease: Grease,
    pub material_a: Material,
    pub material_b: Material,
}

impl LubricatedPair {
    pub fn tabulated_mu_static(&self) -> Option<f64> {
        friction::by_pair(self.material_a.formula, self.material_b.formula, "lubricated")
            .map(|f| f.mu_static)
    }

    pub fn tabulated_mu_kinetic(&self) -> Option<f64> {
        friction::by_pair(self.material_a.formula, self.material_b.formula, "lubricated")
            .map(|f| f.mu_kinetic)
    }

    pub fn effective_mu_kinetic(&self) -> f64 {
        self.tabulated_mu_kinetic()
            .unwrap_or(self.grease.friction_coefficient)
    }

    pub fn friction_force_n(&self, normal_load_n: f64) -> f64 {
        self.effective_mu_kinetic() * normal_load_n
    }

    pub fn archard_wear_constant(&self) -> Option<f64> {
        wear::by_pair(self.material_a.formula, self.material_b.formula, "lubricated")
            .map(|w| w.archard_k)
    }

    pub fn archard_wear_volume_m3(&self, normal_force_n: f64, sliding_distance_m: f64) -> Option<f64> {
        let k = self.archard_wear_constant()?;
        let hv_softer = self
            .material_a
            .vickers_hv()
            .into_iter()
            .chain(self.material_b.vickers_hv())
            .reduce(f64::min)?;
        let hardness_pa = hv_softer * 9.81e6;
        Some(sf_contact::archard_wear_volume(
            k,
            normal_force_n,
            sliding_distance_m,
            hardness_pa,
        ))
    }
}
