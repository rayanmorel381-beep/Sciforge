use crate::moleculars::Material;
use sciforge_hub::prelude::physics::solid_mechanics::stress as sf_st;

#[derive(Debug, Clone, Copy)]
pub struct HertzContact {
    pub material_a: Material,
    pub material_b: Material,
    pub radius_a_m: f64,
    pub radius_b_m: f64,
}

impl HertzContact {
    pub fn new(material_a: Material, material_b: Material, radius_a_m: f64, radius_b_m: f64) -> Self {
        Self {
            material_a,
            material_b,
            radius_a_m,
            radius_b_m,
        }
    }

    pub fn effective_modulus_pa(&self) -> f64 {
        let a = (1.0 - self.material_a.poisson_ratio.powi(2)) / self.material_a.young_modulus_pa;
        let b = (1.0 - self.material_b.poisson_ratio.powi(2)) / self.material_b.young_modulus_pa;
        1.0 / (a + b)
    }

    pub fn effective_radius_m(&self) -> f64 {
        self.radius_a_m * self.radius_b_m / (self.radius_a_m + self.radius_b_m)
    }

    pub fn max_pressure_pa(&self, force_n: f64) -> f64 {
        sf_st::hertz_contact_pressure(
            force_n,
            self.radius_a_m,
            self.radius_b_m,
            self.effective_modulus_pa(),
        )
    }

    pub fn contact_radius_m(&self, force_n: f64) -> f64 {
        let r_eff = self.effective_radius_m();
        let e_star = self.effective_modulus_pa();
        (3.0 * force_n * r_eff / (4.0 * e_star)).powf(1.0 / 3.0)
    }

    pub fn approach_distance_m(&self, force_n: f64) -> f64 {
        let r_eff = self.effective_radius_m();
        let e_star = self.effective_modulus_pa();
        ((9.0 * force_n.powi(2)) / (16.0 * e_star.powi(2) * r_eff)).powf(1.0 / 3.0)
    }

    pub fn yields(&self, force_n: f64) -> bool {
        let p_max = self.max_pressure_pa(force_n);
        let weakest_yield = self
            .material_a
            .yield_strength_pa
            .min(self.material_b.yield_strength_pa);
        p_max > 1.6 * weakest_yield
    }
}
