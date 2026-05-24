use crate::moleculars::{Liquid, Material};
use sciforge_hub::prelude::constants::physics::optics::absorption;
use sciforge_hub::prelude::physics::optics::devices as sf_dev;

impl Material {
    pub fn absorption_coeff_per_m(&self, wavelength_nm: f64) -> Option<f64> {
        absorption::by_formula_wavelength(self.formula, wavelength_nm)
            .map(|a| a.absorption_coeff_per_m)
    }

    pub fn extinction_coefficient_k(&self, wavelength_nm: f64) -> Option<f64> {
        absorption::by_formula_wavelength(self.formula, wavelength_nm)
            .map(|a| a.extinction_coeff_k)
    }

    pub fn penetration_depth_m(&self, wavelength_nm: f64) -> Option<f64> {
        self.absorption_coeff_per_m(wavelength_nm)
            .map(sf_dev::penetration_depth)
    }

    pub fn beer_lambert_intensity(
        &self,
        incident_intensity: f64,
        wavelength_nm: f64,
        thickness_m: f64,
    ) -> Option<f64> {
        let alpha = self.absorption_coeff_per_m(wavelength_nm)?;
        Some(sf_dev::beer_lambert(incident_intensity, alpha, thickness_m))
    }

    pub fn transmittance(&self, wavelength_nm: f64, thickness_m: f64) -> Option<f64> {
        let alpha = self.absorption_coeff_per_m(wavelength_nm)?;
        Some((-alpha * thickness_m).exp())
    }
}

impl Liquid {
    pub fn absorption_coeff_per_m(&self, wavelength_nm: f64) -> Option<f64> {
        absorption::by_formula_wavelength(self.formula, wavelength_nm)
            .map(|a| a.absorption_coeff_per_m)
    }

    pub fn beer_lambert_intensity(
        &self,
        incident_intensity: f64,
        wavelength_nm: f64,
        thickness_m: f64,
    ) -> Option<f64> {
        let alpha = self.absorption_coeff_per_m(wavelength_nm)?;
        Some(sf_dev::beer_lambert(incident_intensity, alpha, thickness_m))
    }

    pub fn transmittance(&self, wavelength_nm: f64, thickness_m: f64) -> Option<f64> {
        let alpha = self.absorption_coeff_per_m(wavelength_nm)?;
        Some((-alpha * thickness_m).exp())
    }
}
