use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::friction;
use sciforge_hub::prelude::constants::physics::solid_mechanics::wear;
use sciforge_hub::prelude::physics::solid_mechanics::contact as sf_contact;

#[derive(Debug, Clone, Copy)]
pub struct Tribology {
    pub material_a: Material,
    pub material_b: Material,
}

impl Tribology {
    pub fn new(material_a: Material, material_b: Material) -> Self {
        Self { material_a, material_b }
    }

    pub fn mu_static(&self, condition: &str) -> Option<f64> {
        friction::by_pair(self.material_a.formula, self.material_b.formula, condition)
            .map(|f| f.mu_static)
    }

    pub fn mu_kinetic(&self, condition: &str) -> Option<f64> {
        friction::by_pair(self.material_a.formula, self.material_b.formula, condition)
            .map(|f| f.mu_kinetic)
    }

    pub fn friction_force_n(
        &self,
        normal_force_n: f64,
        condition: &str,
        moving: bool,
    ) -> Option<f64> {
        let mu = if moving {
            self.mu_kinetic(condition)?
        } else {
            self.mu_static(condition)?
        };
        Some(mu * normal_force_n)
    }

    pub fn archard_wear_constant(&self, condition: &str) -> Option<f64> {
        wear::by_pair(self.material_a.formula, self.material_b.formula, condition)
            .map(|w| w.archard_k)
    }

    pub fn archard_wear_volume_m3(
        &self,
        normal_force_n: f64,
        sliding_distance_m: f64,
        condition: &str,
    ) -> Option<f64> {
        let k = self.archard_wear_constant(condition)?;
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
