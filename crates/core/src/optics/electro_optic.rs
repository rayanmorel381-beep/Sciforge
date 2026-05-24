use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::optics::electro_optic;
use sciforge_hub::prelude::physics::optics::devices as sf_dev;

impl Material {
    pub fn pockels_coefficient_pm_per_v(&self) -> Option<f64> {
        electro_optic::by_formula(self.formula).map(|e| e.pockels_r_pm_per_v)
    }

    pub fn kerr_n2_m2_per_w(&self) -> Option<f64> {
        electro_optic::by_formula(self.formula).map(|e| e.kerr_n2_m2_per_w)
    }

    pub fn pockels_index_change(&self, e_field_v_per_m: f64) -> Option<f64> {
        let eo = electro_optic::by_formula(self.formula)?;
        Some(sf_dev::pockels_index_change(eo.n_ref, eo.pockels_r_pm_per_v, e_field_v_per_m))
    }

    pub fn kerr_index_change(&self, intensity_w_per_m2: f64) -> Option<f64> {
        let eo = electro_optic::by_formula(self.formula)?;
        Some(sf_dev::kerr_index_change(eo.kerr_n2_m2_per_w, intensity_w_per_m2))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ElectroOpticModulator {
    pub material: Material,
    pub length_m: f64,
}

impl ElectroOpticModulator {
    pub fn new(material: Material, length_m: f64) -> Self {
        Self { material, length_m }
    }

    pub fn pockels_phase_shift(&self, e_field_v_per_m: f64, wavelength_m: f64) -> Option<f64> {
        let dn = self.material.pockels_index_change(e_field_v_per_m)?;
        Some(2.0 * std::f64::consts::PI * dn * self.length_m / wavelength_m)
    }

    pub fn half_wave_voltage_v(&self, wavelength_m: f64, gap_m: f64) -> Option<f64> {
        let eo = electro_optic::by_formula(self.material.formula)?;
        let n3 = eo.n_ref.powi(3);
        let r = eo.pockels_r_pm_per_v * 1.0e-12;
        if n3 == 0.0 || r == 0.0 || self.length_m == 0.0 {
            return None;
        }
        Some(wavelength_m * gap_m / (n3 * r * self.length_m))
    }
}
