use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::crystal_elastic;
use sciforge_hub::prelude::constants::physics::solid_mechanics::damping;
use sciforge_hub::prelude::physics::solid_mechanics::vibration as sf_vib;

impl Material {
    pub fn crystal_c11_pa(&self) -> Option<f64> {
        crystal_elastic::by_formula(self.formula).map(|c| c.c11_pa)
    }

    pub fn crystal_c12_pa(&self) -> Option<f64> {
        crystal_elastic::by_formula(self.formula).map(|c| c.c12_pa)
    }

    pub fn crystal_c44_pa(&self) -> Option<f64> {
        crystal_elastic::by_formula(self.formula).map(|c| c.c44_pa)
    }

    pub fn crystal_lattice(&self) -> Option<&'static str> {
        crystal_elastic::by_formula(self.formula).map(|c| c.lattice)
    }

    pub fn anisotropy_zener(&self) -> Option<f64> {
        let entry = crystal_elastic::by_formula(self.formula)?;
        Some(crystal_elastic::anisotropy_zener(entry))
    }

    pub fn loss_factor_eta(&self) -> Option<f64> {
        damping::by_formula(self.formula).map(|d| d.loss_factor_eta)
    }

    pub fn quality_factor_q(&self) -> Option<f64> {
        damping::by_formula(self.formula).map(|d| d.q_factor)
    }

    pub fn damping_ratio_zeta(&self) -> Option<f64> {
        damping::by_formula(self.formula).map(|d| sf_vib::q_factor_to_zeta(d.q_factor))
    }
}

pub fn damped_frequency_rad_per_s(omega_n_rad_per_s: f64, zeta: f64) -> f64 {
    sf_vib::damped_frequency(omega_n_rad_per_s, zeta)
}

pub fn logarithmic_decrement(zeta: f64) -> f64 {
    sf_vib::logarithmic_decrement(zeta)
}

pub fn damping_ratio_from_log_decrement(delta: f64) -> f64 {
    sf_vib::damping_ratio_from_log_decrement(delta)
}
