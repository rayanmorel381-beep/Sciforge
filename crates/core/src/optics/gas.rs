use crate::moleculars::Gas;
use sciforge_hub::prelude::constants::C;
use sciforge_hub::prelude::constants::chemistry::molecular::refractive_index;
use sciforge_hub::prelude::physics::optics::refraction as sf_refr;
use sciforge_hub::prelude::physics::optics::scattering as sf_scat;

impl Gas {
    pub fn refractive_index(&self) -> Option<f64> {
        refractive_index::by_formula_phase(self.formula, "gas").map(|r| r.n_d)
    }

    pub fn speed_of_light_in_m_s(&self) -> Option<f64> {
        self.refractive_index().map(|n| C / n)
    }

    pub fn optical_path_length_m(&self, geometric_length_m: f64) -> Option<f64> {
        self.refractive_index()
            .map(|n| sf_refr::optical_path_length(n, geometric_length_m))
    }

    pub fn snell_refracted_angle_from_vacuum_rad(&self, incidence_rad: f64) -> Option<f64> {
        self.refractive_index()
            .map(|n| sf_refr::snell(1.0, incidence_rad, n))
    }

    pub fn brewster_angle_from_vacuum_rad(&self) -> Option<f64> {
        self.refractive_index().map(|n| sf_refr::brewster_angle(1.0, n))
    }

    pub fn rayleigh_cross_section_m2(&self, wavelength_m: f64, depolarization: f64) -> Option<f64> {
        self.refractive_index()
            .map(|n| sf_scat::rayleigh_cross_section(wavelength_m, n, depolarization))
    }

    pub fn rayleigh_scattering_coefficient_per_m(
        &self,
        wavelength_m: f64,
        depolarization: f64,
    ) -> Option<f64> {
        let n = self.refractive_index()?;
        let number_density = self.density_kg_m3_ref / self.molar_mass_kg_per_mol
            * sciforge_hub::prelude::constants::N_A;
        Some(sf_scat::rayleigh_scattering_coefficient(
            number_density,
            wavelength_m,
            n,
            depolarization,
        ))
    }

    pub fn atmospheric_refraction_arcmin(&self, zenith_angle_rad: f64) -> f64 {
        sf_scat::atmospheric_refraction(zenith_angle_rad, self.pressure_ref_pa, self.temperature_ref_k)
    }

    pub fn absorption_coefficient_per_m(&self, cross_section_m2: f64) -> f64 {
        let number_density = self.density_kg_m3_ref / self.molar_mass_kg_per_mol
            * sciforge_hub::prelude::constants::N_A;
        sf_scat::absorption_coefficient_gas(cross_section_m2, number_density)
    }
}
