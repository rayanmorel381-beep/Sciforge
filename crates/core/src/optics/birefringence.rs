use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::optics::birefringence;
use sciforge_hub::prelude::physics::optics::polarization as sf_pol;

impl Material {
    pub fn ordinary_index(&self) -> Option<f64> {
        birefringence::by_formula(self.formula).map(|b| b.n_ordinary)
    }

    pub fn extraordinary_index(&self) -> Option<f64> {
        birefringence::by_formula(self.formula).map(|b| b.n_extraordinary)
    }

    pub fn birefringence_kind(&self) -> Option<&'static str> {
        birefringence::by_formula(self.formula).map(|b| b.kind)
    }

    pub fn birefringence_value(&self) -> Option<f64> {
        let b = birefringence::by_formula(self.formula)?;
        Some(sf_pol::birefringence(b.n_extraordinary, b.n_ordinary))
    }

    pub fn is_uniaxial(&self) -> bool {
        birefringence::by_formula(self.formula).is_some()
    }

    pub fn retardance_at_m(&self, thickness_m: f64, wavelength_m: f64) -> Option<f64> {
        let dn = self.birefringence_value()?;
        Some(sf_pol::retardance(dn.abs(), thickness_m, wavelength_m))
    }

    pub fn quarter_wave_thickness_m(&self, wavelength_m: f64) -> Option<f64> {
        let dn = self.birefringence_value()?.abs();
        if dn == 0.0 {
            return None;
        }
        Some(wavelength_m / (4.0 * dn))
    }

    pub fn half_wave_thickness_m(&self, wavelength_m: f64) -> Option<f64> {
        let dn = self.birefringence_value()?.abs();
        if dn == 0.0 {
            return None;
        }
        Some(wavelength_m / (2.0 * dn))
    }
}
