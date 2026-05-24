use sciforge_core::materials::nickels::inconel::INCONEL_718;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpoolType {
    LowPressure,
    HighPressure,
    Power,
}

#[derive(Debug, Clone)]
pub struct TurbineShaft {
    pub spool_type: SpoolType,
    pub idle_rpm: u32,
    pub max_rpm: u32,
    pub diameter_m: f64,
    pub material_density_kg_m3: f64,
    pub max_stress_mpa: f64,
}

impl TurbineShaft {
    pub fn high_pressure_spool(max_rpm: u32) -> Self {
        Self {
            spool_type: SpoolType::HighPressure,
            idle_rpm: (max_rpm as f64 * 0.58) as u32,
            max_rpm,
            diameter_m: 0.08,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            max_stress_mpa: 900.0,
        }
    }

    pub fn low_pressure_spool(max_rpm: u32) -> Self {
        Self {
            spool_type: SpoolType::LowPressure,
            idle_rpm: (max_rpm as f64 * 0.35) as u32,
            max_rpm,
            diameter_m: 0.12,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            max_stress_mpa: 750.0,
        }
    }

    pub fn power_spool(max_rpm: u32, diameter_m: f64) -> Self {
        Self {
            spool_type: SpoolType::Power,
            idle_rpm: (max_rpm as f64 * 0.40) as u32,
            max_rpm,
            diameter_m,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            max_stress_mpa: 800.0,
        }
    }

    pub fn critical_speed_rpm(&self) -> f64 {
        let e = 207_000.0e6_f64;
        let i = std::f64::consts::PI * self.diameter_m.powi(4) / 64.0;
        let rho = self.material_density_kg_m3;
        let area = std::f64::consts::PI * (self.diameter_m / 2.0).powi(2);
        let l = 1.0_f64;
        let lambda = 4.730_f64;
        60.0 / (2.0 * std::f64::consts::PI) * (lambda / l).powi(2) * (e * i / (rho * area)).sqrt()
    }
}
