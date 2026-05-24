use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::fatigue;
use sciforge_hub::prelude::physics::solid_mechanics::fatigue as sf_fat;

impl Material {
    pub fn fatigue_sigma_f_prime_pa(&self) -> Option<f64> {
        fatigue::by_formula(self.formula).map(|f| f.sigma_f_prime_pa)
    }

    pub fn fatigue_basquin_b(&self) -> Option<f64> {
        fatigue::by_formula(self.formula).map(|f| f.basquin_b)
    }

    pub fn fatigue_epsilon_f_prime(&self) -> Option<f64> {
        fatigue::by_formula(self.formula).map(|f| f.epsilon_f_prime)
    }

    pub fn fatigue_coffin_manson_c(&self) -> Option<f64> {
        fatigue::by_formula(self.formula).map(|f| f.coffin_manson_c)
    }

    pub fn endurance_limit_pa(&self) -> Option<f64> {
        fatigue::by_formula(self.formula).map(|f| f.endurance_limit_pa)
    }

    pub fn fatigue_ultimate_pa(&self) -> Option<f64> {
        fatigue::by_formula(self.formula).map(|f| f.ultimate_pa)
    }

    pub fn goodman_safety_factor(&self, sigma_a_pa: f64, sigma_m_pa: f64) -> Option<f64> {
        let f = fatigue::by_formula(self.formula)?;
        Some(sf_fat::goodman_criterion(
            sigma_a_pa,
            sigma_m_pa,
            f.endurance_limit_pa,
            f.ultimate_pa,
        ))
    }

    pub fn soderberg_safety_factor(&self, sigma_a_pa: f64, sigma_m_pa: f64) -> Option<f64> {
        let f = fatigue::by_formula(self.formula)?;
        Some(sf_fat::soderberg_criterion(
            sigma_a_pa,
            sigma_m_pa,
            f.endurance_limit_pa,
            self.yield_strength_pa,
        ))
    }

    pub fn gerber_safety_factor(&self, sigma_a_pa: f64, sigma_m_pa: f64) -> Option<f64> {
        let f = fatigue::by_formula(self.formula)?;
        Some(sf_fat::gerber_criterion(
            sigma_a_pa,
            sigma_m_pa,
            f.endurance_limit_pa,
            f.ultimate_pa,
        ))
    }

    pub fn morrow_corrected_amplitude_pa(&self, sigma_a_pa: f64, sigma_m_pa: f64) -> Option<f64> {
        let f = fatigue::by_formula(self.formula)?;
        Some(sf_fat::morrow_correction(
            sigma_a_pa,
            sigma_m_pa,
            f.sigma_f_prime_pa,
        ))
    }

    pub fn smith_watson_topper_parameter(
        &self,
        sigma_max_pa: f64,
        epsilon_a: f64,
    ) -> f64 {
        sf_fat::swt_parameter(sigma_max_pa, epsilon_a, self.young_modulus_pa)
    }

    pub fn walker_corrected_amplitude_pa(
        &self,
        sigma_a_pa: f64,
        sigma_max_pa: f64,
        gamma: f64,
    ) -> f64 {
        sf_fat::walker_correction(sigma_a_pa, sigma_max_pa, gamma)
    }

    pub fn fatigue_life_basquin_cycles(&self, sigma_a_pa: f64) -> Option<f64> {
        let f = fatigue::by_formula(self.formula)?;
        Some(sf_fat::sn_curve_basquin_inverse(
            sigma_a_pa,
            f.sigma_f_prime_pa,
            f.basquin_b,
        ))
    }

    pub fn marin_endurance_factor(
        &self,
        k_load: f64,
        k_size: f64,
        k_surface: f64,
        k_temperature: f64,
        k_reliability: f64,
        k_misc: f64,
    ) -> f64 {
        sf_fat::endurance_factor_marin(
            k_load,
            k_size,
            k_surface,
            k_temperature,
            k_reliability,
            k_misc,
        )
    }
}
