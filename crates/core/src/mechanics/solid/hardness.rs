use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::hardness;

impl Material {
    pub fn vickers_hv(&self) -> Option<f64> {
        hardness::by_material(self.formula)
            .and_then(|h| h.vickers_hv)
            .or({
                if self.hardness_hv > 0.0 {
                    Some(self.hardness_hv)
                } else {
                    None
                }
            })
    }

    pub fn brinell_hb(&self) -> Option<f64> {
        hardness::by_material(self.formula).and_then(|h| h.brinell_hb)
    }

    pub fn mohs(&self) -> Option<f64> {
        hardness::by_material(self.formula).and_then(|h| h.mohs)
    }

    pub fn vickers_to_yield_strength_pa(&self) -> Option<f64> {
        self.vickers_hv().map(|hv| hv * 9.81e6 / 3.0)
    }

    pub fn scratches(&self, other: &Material) -> Option<bool> {
        let a = self.mohs()?;
        let b = other.mohs()?;
        Some(a > b)
    }
}
