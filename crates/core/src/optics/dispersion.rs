use crate::moleculars::{Gas, Liquid, Material};
use sciforge_hub::prelude::constants::physics::optics::dispersion;
use sciforge_hub::prelude::physics::optics::devices as sf_dev;

impl Material {
    pub fn sellmeier_n_at_um(&self, wavelength_um: f64) -> Option<f64> {
        let s = dispersion::by_formula(self.formula)?;
        Some(sf_dev::sellmeier(
            s.b1, s.c1_um2, s.b2, s.c2_um2, s.b3, s.c3_um2, wavelength_um,
        ))
    }

    pub fn sellmeier_n_at_nm(&self, wavelength_nm: f64) -> Option<f64> {
        self.sellmeier_n_at_um(wavelength_nm * 1.0e-3)
    }

    pub fn sellmeier_valid_range_um(&self) -> Option<(f64, f64)> {
        dispersion::by_formula(self.formula).map(|s| s.valid_range_um)
    }

    pub fn is_sellmeier_in_range_um(&self, wavelength_um: f64) -> Option<bool> {
        let (lo, hi) = self.sellmeier_valid_range_um()?;
        Some((lo..=hi).contains(&wavelength_um))
    }

    pub fn abbe_number(&self) -> Option<f64> {
        let n_d = self.sellmeier_n_at_nm(587.6)?;
        let n_f = self.sellmeier_n_at_nm(486.1)?;
        let n_c = self.sellmeier_n_at_nm(656.3)?;
        Some(sf_dev::abbe_number(n_d, n_f, n_c))
    }

    pub fn group_index_at_um(&self, wavelength_um: f64) -> Option<f64> {
        let h = 1.0e-3;
        let n0 = self.sellmeier_n_at_um(wavelength_um)?;
        let n_plus = self.sellmeier_n_at_um(wavelength_um + h)?;
        let n_minus = self.sellmeier_n_at_um(wavelength_um - h)?;
        let dn_dlambda_per_m = (n_plus - n_minus) / (2.0 * h * 1.0e-6);
        Some(sf_dev::group_index(n0, wavelength_um * 1.0e-6, dn_dlambda_per_m))
    }
}

impl Liquid {
    pub fn sellmeier_n_at_um(&self, wavelength_um: f64) -> Option<f64> {
        let s = dispersion::by_formula(self.formula)?;
        Some(sf_dev::sellmeier(
            s.b1, s.c1_um2, s.b2, s.c2_um2, s.b3, s.c3_um2, wavelength_um,
        ))
    }
}

impl Gas {
    pub fn sellmeier_n_at_um(&self, wavelength_um: f64) -> Option<f64> {
        let s = dispersion::by_formula(self.formula)?;
        Some(sf_dev::sellmeier(
            s.b1, s.c1_um2, s.b2, s.c2_um2, s.b3, s.c3_um2, wavelength_um,
        ))
    }
}
