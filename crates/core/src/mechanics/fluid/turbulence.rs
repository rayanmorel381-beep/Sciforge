use crate::moleculars::{Gas, Liquid};
use sciforge_hub::prelude::physics::fluid_mechanics::turbulence as sf_turb;

impl Gas {
    pub fn kinematic_viscosity_m2_s(&self, pressure_pa: f64, temperature_k: f64) -> f64 {
        self.dynamic_viscosity_pa_s(temperature_k)
            / self.density_ideal_kg_m3(pressure_pa, temperature_k)
    }

    pub fn friction_velocity_m_s(&self, wall_shear_stress_pa: f64, pressure_pa: f64, temperature_k: f64) -> f64 {
        sf_turb::friction_velocity(
            wall_shear_stress_pa,
            self.density_ideal_kg_m3(pressure_pa, temperature_k),
        )
    }

    pub fn kolmogorov_length_scale_m(&self, dissipation_rate_m2_s3: f64, pressure_pa: f64, temperature_k: f64) -> f64 {
        sf_turb::kolmogorov_length_scale(
            self.kinematic_viscosity_m2_s(pressure_pa, temperature_k),
            dissipation_rate_m2_s3,
        )
    }

    pub fn kolmogorov_time_scale_s(&self, dissipation_rate_m2_s3: f64, pressure_pa: f64, temperature_k: f64) -> f64 {
        sf_turb::kolmogorov_time_scale(
            self.kinematic_viscosity_m2_s(pressure_pa, temperature_k),
            dissipation_rate_m2_s3,
        )
    }

    pub fn kolmogorov_velocity_scale_m_s(&self, dissipation_rate_m2_s3: f64, pressure_pa: f64, temperature_k: f64) -> f64 {
        sf_turb::kolmogorov_velocity_scale(
            self.kinematic_viscosity_m2_s(pressure_pa, temperature_k),
            dissipation_rate_m2_s3,
        )
    }

    pub fn taylor_microscale_m(
        &self,
        u_rms_m_s: f64,
        dissipation_rate_m2_s3: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        sf_turb::taylor_microscale(
            u_rms_m_s,
            dissipation_rate_m2_s3,
            self.kinematic_viscosity_m2_s(pressure_pa, temperature_k),
        )
    }

    pub fn law_of_wall_velocity_m_s(
        &self,
        wall_shear_stress_pa: f64,
        wall_distance_m: f64,
        pressure_pa: f64,
        temperature_k: f64,
    ) -> f64 {
        let u_tau = self.friction_velocity_m_s(wall_shear_stress_pa, pressure_pa, temperature_k);
        sf_turb::law_of_wall(u_tau, wall_distance_m, self.kinematic_viscosity_m2_s(pressure_pa, temperature_k))
    }
}

impl Liquid {
    pub fn friction_velocity_m_s(&self, wall_shear_stress_pa: f64) -> f64 {
        sf_turb::friction_velocity(wall_shear_stress_pa, self.density_kg_m3_ref)
    }

    pub fn kolmogorov_length_scale_m(&self, dissipation_rate_m2_s3: f64) -> f64 {
        sf_turb::kolmogorov_length_scale(self.kinematic_viscosity_m2_s(), dissipation_rate_m2_s3)
    }

    pub fn kolmogorov_time_scale_s(&self, dissipation_rate_m2_s3: f64) -> f64 {
        sf_turb::kolmogorov_time_scale(self.kinematic_viscosity_m2_s(), dissipation_rate_m2_s3)
    }

    pub fn kolmogorov_velocity_scale_m_s(&self, dissipation_rate_m2_s3: f64) -> f64 {
        sf_turb::kolmogorov_velocity_scale(self.kinematic_viscosity_m2_s(), dissipation_rate_m2_s3)
    }

    pub fn taylor_microscale_m(&self, u_rms_m_s: f64, dissipation_rate_m2_s3: f64) -> f64 {
        sf_turb::taylor_microscale(u_rms_m_s, dissipation_rate_m2_s3, self.kinematic_viscosity_m2_s())
    }

    pub fn law_of_wall_velocity_m_s(&self, wall_shear_stress_pa: f64, wall_distance_m: f64) -> f64 {
        let u_tau = self.friction_velocity_m_s(wall_shear_stress_pa);
        sf_turb::law_of_wall(u_tau, wall_distance_m, self.kinematic_viscosity_m2_s())
    }
}
