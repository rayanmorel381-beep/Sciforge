use crate::moleculars::Material;
use sciforge_hub::prelude::physics::solid_mechanics::elasticity as sf_el;

impl Material {
    pub fn shear_modulus_pa(&self) -> f64 {
        sf_el::shear_modulus(self.young_modulus_pa, self.poisson_ratio)
    }

    pub fn bulk_modulus_pa(&self) -> f64 {
        sf_el::bulk_modulus(self.young_modulus_pa, self.poisson_ratio)
    }

    pub fn lame_first_pa(&self) -> f64 {
        sf_el::lame_first(self.young_modulus_pa, self.poisson_ratio)
    }

    pub fn hooke_stress_pa(&self, strain: f64) -> f64 {
        sf_el::hooke_stress(self.young_modulus_pa, strain)
    }

    pub fn hooke_strain(&self, stress_pa: f64) -> f64 {
        sf_el::hooke_strain(stress_pa, self.young_modulus_pa)
    }

    pub fn lateral_strain(&self, axial_strain: f64) -> f64 {
        sf_el::poisson_lateral_strain(axial_strain, self.poisson_ratio)
    }

    pub fn plane_stress_strain(&self, stress_x_pa: f64, stress_y_pa: f64) -> (f64, f64) {
        sf_el::plane_stress_strain(
            stress_x_pa,
            stress_y_pa,
            self.young_modulus_pa,
            self.poisson_ratio,
        )
    }

    pub fn strain_energy_density_j_per_m3(&self, stress_pa: f64) -> f64 {
        let strain = sf_el::hooke_strain(stress_pa, self.young_modulus_pa);
        sf_el::strain_energy_density(stress_pa, strain)
    }

    pub fn elastic_wave_speed_longitudinal_m_s(&self) -> f64 {
        let lambda = self.lame_first_pa();
        let g = self.shear_modulus_pa();
        ((lambda + 2.0 * g) / self.density_kg_m3).sqrt()
    }

    pub fn elastic_wave_speed_shear_m_s(&self) -> f64 {
        (self.shear_modulus_pa() / self.density_kg_m3).sqrt()
    }

    pub fn pugh_ratio(&self) -> f64 {
        self.bulk_modulus_pa() / self.shear_modulus_pa()
    }

    pub fn is_ductile_pugh(&self) -> bool {
        self.pugh_ratio() > 1.75
    }
}
