use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::elastic_moduli;

impl Material {
    pub fn young_modulus_table_pa(&self) -> Option<f64> {
        elastic_moduli::by_formula(self.formula).map(|e| e.young_pa)
    }

    pub fn shear_modulus_table_pa(&self) -> Option<f64> {
        elastic_moduli::by_formula(self.formula).map(|e| e.shear_pa)
    }

    pub fn bulk_modulus_table_pa(&self) -> Option<f64> {
        elastic_moduli::by_formula(self.formula).map(|e| e.bulk_pa)
    }

    pub fn poisson_ratio_table(&self) -> Option<f64> {
        elastic_moduli::by_formula(self.formula).map(|e| e.poisson)
    }
}
