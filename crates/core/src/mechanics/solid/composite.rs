use crate::moleculars::Material;
use sciforge_hub::prelude::physics::solid_mechanics::composites as sf_comp;

#[derive(Debug, Clone, Copy)]
pub struct Composite {
    pub fiber: Material,
    pub matrix: Material,
    pub volume_fraction_fiber: f64,
}

impl Composite {
    pub fn new(fiber: Material, matrix: Material, volume_fraction_fiber: f64) -> Self {
        Self {
            fiber,
            matrix,
            volume_fraction_fiber,
        }
    }

    pub fn voigt_modulus_pa(&self) -> f64 {
        sf_comp::voigt_modulus(
            self.fiber.young_modulus_pa,
            self.matrix.young_modulus_pa,
            self.volume_fraction_fiber,
        )
    }

    pub fn reuss_modulus_pa(&self) -> f64 {
        sf_comp::reuss_modulus(
            self.fiber.young_modulus_pa,
            self.matrix.young_modulus_pa,
            self.volume_fraction_fiber,
        )
    }

    pub fn halpin_tsai_modulus_pa(&self, xi: f64) -> f64 {
        sf_comp::halpin_tsai(
            self.fiber.young_modulus_pa,
            self.matrix.young_modulus_pa,
            self.volume_fraction_fiber,
            xi,
        )
    }

    pub fn density_kg_m3(&self) -> f64 {
        sf_comp::rule_of_mixtures_density(
            self.fiber.density_kg_m3,
            self.matrix.density_kg_m3,
            self.volume_fraction_fiber,
        )
    }

    pub fn hashin_shtrikman_bounds_pa(&self) -> (f64, f64) {
        sf_comp::hashin_shtrikman_bounds(
            self.fiber.bulk_modulus_pa(),
            self.matrix.bulk_modulus_pa(),
            self.fiber.shear_modulus_pa(),
            self.matrix.shear_modulus_pa(),
            self.volume_fraction_fiber,
        )
    }
}
