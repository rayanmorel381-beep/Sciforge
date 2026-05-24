use crate::moleculars::{Gas, Liquid};
use sciforge_hub::prelude::constants::EARTH_GRAVITY;
use sciforge_hub::prelude::physics::fluid_mechanics::flow as sf_flow;

impl Gas {
    pub fn bernoulli_pressure_pa(
        &self,
        v1_m_s: f64,
        p1_pa: f64,
        v2_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_flow::bernoulli_pressure(
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
            v1_m_s,
            p1_pa,
            v2_m_s,
        )
    }

    pub fn bernoulli_height_m(
        &self,
        v1_m_s: f64,
        p1_pa: f64,
        h1_m: f64,
        v2_m_s: f64,
        p2_pa: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_flow::bernoulli_height(
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
            v1_m_s,
            p1_pa,
            h1_m,
            v2_m_s,
            p2_pa,
            EARTH_GRAVITY,
        )
    }

    pub fn hagen_poiseuille_flow_m3_s(
        &self,
        delta_p_pa: f64,
        radius_m: f64,
        length_m: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_flow::hagen_poiseuille(
            delta_p_pa,
            radius_m,
            length_m,
            self.dynamic_viscosity_pa_s(temperature_k),
        )
    }

    pub fn drag_force_n(
        &self,
        cd: f64,
        velocity_m_s: f64,
        area_m2: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_flow::drag_force(
            cd,
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
            velocity_m_s,
            area_m2,
        )
    }

    pub fn lift_force_n(
        &self,
        cl: f64,
        velocity_m_s: f64,
        area_m2: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_flow::lift_force(
            cl,
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
            velocity_m_s,
            area_m2,
        )
    }

    pub fn stokes_drag_n(&self, radius_m: f64, velocity_m_s: f64, temperature_k: f64) -> f64 {
        sf_flow::stokes_drag(
            self.dynamic_viscosity_pa_s(temperature_k),
            radius_m,
            velocity_m_s,
        )
    }

    pub fn darcy_weisbach_pressure_drop_pa(
        &self,
        friction_factor: f64,
        length_m: f64,
        diameter_m: f64,
        velocity_m_s: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_flow::darcy_weisbach(
            friction_factor,
            length_m,
            diameter_m,
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
            velocity_m_s,
        )
    }
}

impl Liquid {
    pub fn bernoulli_pressure_pa(&self, v1_m_s: f64, p1_pa: f64, v2_m_s: f64) -> f64 {
        sf_flow::bernoulli_pressure(self.density_kg_m3_ref, v1_m_s, p1_pa, v2_m_s)
    }

    pub fn hagen_poiseuille_flow_m3_s(
        &self,
        delta_p_pa: f64,
        radius_m: f64,
        length_m: f64,
    ) -> f64 {
        sf_flow::hagen_poiseuille(
            delta_p_pa,
            radius_m,
            length_m,
            self.dynamic_viscosity_pa_s_ref,
        )
    }

    pub fn drag_force_n(&self, cd: f64, velocity_m_s: f64, area_m2: f64) -> f64 {
        sf_flow::drag_force(cd, self.density_kg_m3_ref, velocity_m_s, area_m2)
    }

    pub fn lift_force_n(&self, cl: f64, velocity_m_s: f64, area_m2: f64) -> f64 {
        sf_flow::lift_force(cl, self.density_kg_m3_ref, velocity_m_s, area_m2)
    }

    pub fn stokes_drag_n(&self, radius_m: f64, velocity_m_s: f64) -> f64 {
        sf_flow::stokes_drag(self.dynamic_viscosity_pa_s_ref, radius_m, velocity_m_s)
    }

    pub fn terminal_velocity_sphere_m_s(
        &self,
        particle_density_kg_m3: f64,
        particle_radius_m: f64,
    ) -> f64 {
        sf_flow::terminal_velocity_sphere(
            particle_density_kg_m3,
            self.density_kg_m3_ref,
            particle_radius_m,
            self.dynamic_viscosity_pa_s_ref,
            EARTH_GRAVITY,
        )
    }

    pub fn darcy_weisbach_pressure_drop_pa(
        &self,
        friction_factor: f64,
        length_m: f64,
        diameter_m: f64,
        velocity_m_s: f64,
    ) -> f64 {
        sf_flow::darcy_weisbach(
            friction_factor,
            length_m,
            diameter_m,
            self.density_kg_m3_ref,
            velocity_m_s,
        )
    }

    pub fn torricelli_velocity_m_s(&self, head_m: f64) -> f64 {
        sf_flow::torricelli(EARTH_GRAVITY, head_m)
    }
}
