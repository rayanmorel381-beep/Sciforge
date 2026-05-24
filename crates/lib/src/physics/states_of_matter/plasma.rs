use crate::constants::physics::elements::Element;
use crate::constants::physics::{E_CHARGE, ELECTRON_MASS_KG, EPSILON_0, K_B, N_A};

#[derive(Debug, Clone, Copy)]
pub struct Plasma {
    pub element: Option<&'static Element>,
    pub electron_temperature_k: f64,
    pub ion_temperature_k: f64,
    pub electron_density_per_m3: f64,
    pub ionization_degree: f64,
    pub mean_ion_charge: f64,
    pub molar_mass_kg_per_mol: f64,
}

impl Plasma {
    pub fn debye_length_m(&self) -> f64 {
        (EPSILON_0 * K_B * self.electron_temperature_k
            / (self.electron_density_per_m3 * E_CHARGE * E_CHARGE))
            .sqrt()
    }

    pub fn plasma_frequency_rad_s(&self) -> f64 {
        (self.electron_density_per_m3 * E_CHARGE * E_CHARGE / (EPSILON_0 * ELECTRON_MASS_KG)).sqrt()
    }

    pub fn coupling_parameter(&self) -> f64 {
        let pi = core::f64::consts::PI;
        let a_ws = (3.0 / (4.0 * pi * self.electron_density_per_m3)).cbrt();
        let q2 = self.mean_ion_charge * self.mean_ion_charge * E_CHARGE * E_CHARGE;
        q2 / (4.0 * pi * EPSILON_0 * a_ws * K_B * self.electron_temperature_k)
    }

    pub fn thermal_speed_electron_m_s(&self) -> f64 {
        (K_B * self.electron_temperature_k / ELECTRON_MASS_KG).sqrt()
    }

    pub fn thermal_speed_ion_m_s(&self) -> f64 {
        let m_i = self.molar_mass_kg_per_mol / N_A;
        (K_B * self.ion_temperature_k / m_i).sqrt()
    }

    pub fn ion_density_per_m3(&self) -> f64 {
        self.electron_density_per_m3 / self.mean_ion_charge.max(1.0)
    }

    pub fn neutral_density_per_m3(&self) -> f64 {
        let alpha = self.ionization_degree.clamp(1.0e-12, 1.0);
        self.ion_density_per_m3() * (1.0 - alpha) / alpha
    }
}
