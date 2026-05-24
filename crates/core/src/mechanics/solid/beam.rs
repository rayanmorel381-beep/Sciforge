use crate::moleculars::Material;
use sciforge_hub::prelude::physics::solid_mechanics::buckling as sf_buck;
use sciforge_hub::prelude::physics::solid_mechanics::stress as sf_st;
use sciforge_hub::prelude::physics::solid_mechanics::vibration as sf_vib;

#[derive(Debug, Clone, Copy)]
pub struct Beam {
    pub material: Material,
    pub length_m: f64,
    pub second_moment_area_m4: f64,
    pub section_modulus_m3: f64,
}

impl Beam {
    pub fn new(
        material: Material,
        length_m: f64,
        second_moment_area_m4: f64,
        section_modulus_m3: f64,
    ) -> Self {
        Self {
            material,
            length_m,
            second_moment_area_m4,
            section_modulus_m3,
        }
    }

    pub fn rectangular(material: Material, length_m: f64, width_m: f64, height_m: f64) -> Self {
        let i = width_m * height_m.powi(3) / 12.0;
        let s = width_m * height_m.powi(2) / 6.0;
        Self::new(material, length_m, i, s)
    }

    pub fn circular(material: Material, length_m: f64, radius_m: f64) -> Self {
        let i = std::f64::consts::PI * radius_m.powi(4) / 4.0;
        let s = std::f64::consts::PI * radius_m.powi(3) / 4.0;
        Self::new(material, length_m, i, s)
    }

    pub fn bending_stress_pa(&self, moment_n_m: f64, fiber_distance_m: f64) -> f64 {
        sf_st::beam_bending_stress(moment_n_m, fiber_distance_m, self.second_moment_area_m4)
    }

    pub fn max_bending_stress_pa(&self, moment_n_m: f64) -> f64 {
        moment_n_m / self.section_modulus_m3
    }

    pub fn cantilever_tip_deflection_m(&self, point_load_n: f64) -> f64 {
        sf_st::beam_deflection_cantilever(
            point_load_n,
            self.length_m,
            self.material.young_modulus_pa,
            self.second_moment_area_m4,
        )
    }

    pub fn euler_buckling_load_n(&self) -> f64 {
        sf_st::column_euler_buckling(
            self.material.young_modulus_pa,
            self.second_moment_area_m4,
            self.length_m,
        )
    }

    pub fn slenderness_ratio(&self, area_m2: f64) -> f64 {
        let radius_gyration = (self.second_moment_area_m4 / area_m2).sqrt();
        self.length_m / radius_gyration
    }

    pub fn allowable_moment_n_m(&self) -> f64 {
        self.material.yield_strength_pa * self.section_modulus_m3
    }

    pub fn euler_critical_stress_pa(&self, area_m2: f64, end_condition: &str) -> f64 {
        let l_eff = sf_buck::effective_length(self.length_m, end_condition);
        let r_g = (self.second_moment_area_m4 / area_m2).sqrt();
        sf_buck::euler_critical_stress(self.material.young_modulus_pa, l_eff / r_g)
    }

    pub fn johnson_critical_stress_pa(&self, area_m2: f64, end_condition: &str) -> f64 {
        let l_eff = sf_buck::effective_length(self.length_m, end_condition);
        let r_g = (self.second_moment_area_m4 / area_m2).sqrt();
        sf_buck::johnson_short_column(
            self.material.yield_strength_pa,
            self.material.young_modulus_pa,
            l_eff / r_g,
        )
    }

    pub fn natural_frequency_hz(&self, mode: u32, area_m2: f64, end_conditions: &str) -> f64 {
        let rho_a = self.material.density_kg_m3 * area_m2;
        sf_vib::beam_natural_frequency(
            mode,
            self.material.young_modulus_pa,
            self.second_moment_area_m4,
            rho_a,
            self.length_m,
            end_conditions,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Shaft {
    pub material: Material,
    pub length_m: f64,
    pub polar_moment_m4: f64,
    pub outer_radius_m: f64,
}

impl Shaft {
    pub fn solid_circular(material: Material, length_m: f64, radius_m: f64) -> Self {
        Self {
            material,
            length_m,
            polar_moment_m4: std::f64::consts::PI * radius_m.powi(4) / 2.0,
            outer_radius_m: radius_m,
        }
    }

    pub fn torsion_shear_stress_pa(&self, torque_n_m: f64) -> f64 {
        sf_st::torsion_shear_stress(torque_n_m, self.outer_radius_m, self.polar_moment_m4)
    }

    pub fn allowable_torque_n_m(&self) -> f64 {
        let shear_yield = self.material.yield_strength_pa / 3_f64.sqrt();
        shear_yield * self.polar_moment_m4 / self.outer_radius_m
    }

    pub fn angle_of_twist_rad(&self, torque_n_m: f64) -> f64 {
        torque_n_m * self.length_m / (self.material.shear_modulus_pa() * self.polar_moment_m4)
    }
}
