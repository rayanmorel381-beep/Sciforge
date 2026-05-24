use crate::moleculars::Material;
use sciforge_hub::prelude::constants::physics::solid_mechanics::toughness;
use sciforge_hub::prelude::physics::solid_mechanics::fracture as sf_fr;

impl Material {
    pub fn fracture_toughness_pa_sqrt_m(&self) -> Option<f64> {
        toughness::by_material(self.formula).map(|t| t.k_ic_pa_sqrt_m)
    }

    pub fn stress_intensity_factor_pa_sqrt_m(
        &self,
        applied_stress_pa: f64,
        crack_half_length_m: f64,
        geometry_factor: f64,
    ) -> f64 {
        sf_fr::stress_intensity_factor(applied_stress_pa, crack_half_length_m, geometry_factor)
    }

    pub fn will_fracture(
        &self,
        applied_stress_pa: f64,
        crack_half_length_m: f64,
        geometry_factor: f64,
    ) -> Option<bool> {
        let k = sf_fr::stress_intensity_factor(
            applied_stress_pa,
            crack_half_length_m,
            geometry_factor,
        );
        self.fracture_toughness_pa_sqrt_m().map(|kic| k > kic)
    }

    pub fn critical_crack_length_m(
        &self,
        applied_stress_pa: f64,
        geometry_factor: f64,
    ) -> Option<f64> {
        let kic = self.fracture_toughness_pa_sqrt_m()?;
        let denom = geometry_factor * applied_stress_pa;
        if denom <= 0.0 {
            return None;
        }
        let arg = kic / denom;
        Some(arg * arg / std::f64::consts::PI)
    }

    pub fn griffith_critical_stress_pa(
        &self,
        surface_energy_j_per_m2: f64,
        crack_half_length_m: f64,
    ) -> f64 {
        sf_fr::griffith_critical_stress(
            self.young_modulus_pa,
            surface_energy_j_per_m2,
            crack_half_length_m,
        )
    }

    pub fn energy_release_rate_j_per_m2(&self, k_pa_sqrt_m: f64) -> f64 {
        sf_fr::energy_release_rate(k_pa_sqrt_m, self.young_modulus_pa)
    }

    pub fn crack_tip_plastic_zone_m(&self, k_pa_sqrt_m: f64) -> f64 {
        sf_fr::crack_tip_plastic_zone(k_pa_sqrt_m, self.yield_strength_pa)
    }

    pub fn crack_opening_displacement_m(&self, k_pa_sqrt_m: f64) -> f64 {
        sf_fr::crack_opening_displacement(k_pa_sqrt_m, self.yield_strength_pa, self.young_modulus_pa)
    }

    pub fn paris_crack_growth_m_per_cycle(
        &self,
        c_paris: f64,
        delta_k_pa_sqrt_m: f64,
        m_paris: f64,
    ) -> f64 {
        sf_fr::paris_law(c_paris, delta_k_pa_sqrt_m, m_paris)
    }

    pub fn fatigue_life_cycles(&self, stress_amplitude_pa: f64) -> f64 {
        sf_fr::fatigue_life_basquin(
            self.fatigue_strength_coeff_pa,
            stress_amplitude_pa,
            self.fatigue_strength_exponent,
        )
    }

    pub fn fatigue_life_coffin_manson_cycles(
        &self,
        ductility_coeff: f64,
        plastic_strain_amplitude: f64,
        ductility_exponent: f64,
    ) -> f64 {
        sf_fr::fatigue_life_coffin_manson(
            ductility_coeff,
            plastic_strain_amplitude,
            ductility_exponent,
        )
    }

    pub fn miner_damage(&self, cycles: &[f64], max_cycles: &[f64]) -> f64 {
        sf_fr::miners_rule(cycles, max_cycles)
    }
}
