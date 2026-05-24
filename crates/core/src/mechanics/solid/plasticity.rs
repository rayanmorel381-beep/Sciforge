use crate::moleculars::Material;
use sciforge_hub::prelude::physics::solid_mechanics::plasticity as sf_pl;

impl Material {
    pub fn von_mises_stress_pa(&self, s1_pa: f64, s2_pa: f64, s3_pa: f64) -> f64 {
        sf_pl::von_mises(s1_pa, s2_pa, s3_pa)
    }

    pub fn tresca_stress_pa(&self, s1_pa: f64, s2_pa: f64, s3_pa: f64) -> f64 {
        sf_pl::tresca(s1_pa, s2_pa, s3_pa)
    }

    pub fn yields_von_mises(&self, s1_pa: f64, s2_pa: f64, s3_pa: f64) -> bool {
        sf_pl::von_mises(s1_pa, s2_pa, s3_pa) > self.yield_strength_pa
    }

    pub fn yields_tresca(&self, s1_pa: f64, s2_pa: f64, s3_pa: f64) -> bool {
        sf_pl::tresca(s1_pa, s2_pa, s3_pa) > self.yield_strength_pa
    }

    pub fn safety_factor_von_mises(&self, s1_pa: f64, s2_pa: f64, s3_pa: f64) -> f64 {
        let sigma_eq = sf_pl::von_mises(s1_pa, s2_pa, s3_pa);
        if sigma_eq == 0.0 {
            f64::INFINITY
        } else {
            self.yield_strength_pa / sigma_eq
        }
    }

    pub fn ramberg_osgood_strain(&self, stress_pa: f64, k_pa: f64, n: f64) -> f64 {
        sf_pl::ramberg_osgood(stress_pa, self.young_modulus_pa, k_pa, n)
    }

    pub fn true_stress_pa(&self, engineering_stress_pa: f64, engineering_strain: f64) -> f64 {
        sf_pl::true_stress(engineering_stress_pa, engineering_strain)
    }

    pub fn true_strain(&self, engineering_strain: f64) -> f64 {
        sf_pl::true_strain(engineering_strain)
    }

    pub fn hardening_stress_pa(&self, k_pa: f64, plastic_strain: f64, n: f64) -> f64 {
        sf_pl::hardening_power_law(k_pa, plastic_strain, n)
    }

    pub fn isotropic_hardening_pa(&self, hardening_modulus_pa: f64, plastic_strain: f64) -> f64 {
        sf_pl::isotropic_hardening(self.yield_strength_pa, hardening_modulus_pa, plastic_strain)
    }

    pub fn hardening_exponent_estimate(&self) -> f64 {
        if self.yield_strength_pa <= 0.0 || self.ultimate_strength_pa <= self.yield_strength_pa {
            0.0
        } else {
            (self.ultimate_strength_pa / self.yield_strength_pa).ln()
        }
    }
}
