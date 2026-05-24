use sciforge_core::materials::nickels::inconel::INCONEL_718;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbineStageType {
    HighPressure,
    LowPressure,
    PowerTurbine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbineBladeMaterial {
    Inconel718,
    Cmsx4,
    Cm247,
    TiAl,
}

impl TurbineBladeMaterial {
    pub fn density_kg_m3(self) -> f64 {
        match self {
            TurbineBladeMaterial::Inconel718 => INCONEL_718.density_kg_m3,
            TurbineBladeMaterial::Cmsx4 => 8_700.0,
            TurbineBladeMaterial::Cm247 => 8_530.0,
            TurbineBladeMaterial::TiAl => TI6AL4V_GR5.density_kg_m3,
        }
    }

    pub fn max_service_temp_k(self) -> f64 {
        match self {
            TurbineBladeMaterial::Inconel718 => 1_173.0,
            TurbineBladeMaterial::Cmsx4 => 1_423.0,
            TurbineBladeMaterial::Cm247 => 1_373.0,
            TurbineBladeMaterial::TiAl => 1_073.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TurbineStage {
    pub stage_type: TurbineStageType,
    pub blade_count: u8,
    pub inlet_temp_k: f64,
    pub pressure_ratio: f64,
    pub isentropic_efficiency: f64,
    pub cooling_air_fraction: f64,
    pub blade_material: TurbineBladeMaterial,
}

impl TurbineStage {
    pub fn high_pressure(inlet_temp_k: f64, pressure_ratio: f64) -> Self {
        Self {
            stage_type: TurbineStageType::HighPressure,
            blade_count: 80,
            inlet_temp_k,
            pressure_ratio,
            isentropic_efficiency: 0.88,
            cooling_air_fraction: 0.15,
            blade_material: TurbineBladeMaterial::Cmsx4,
        }
    }

    pub fn low_pressure(inlet_temp_k: f64, pressure_ratio: f64) -> Self {
        Self {
            stage_type: TurbineStageType::LowPressure,
            blade_count: 120,
            inlet_temp_k,
            pressure_ratio,
            isentropic_efficiency: 0.91,
            cooling_air_fraction: 0.04,
            blade_material: TurbineBladeMaterial::Inconel718,
        }
    }

    pub fn power_turbine(shaft_power_kw: f64, mass_flow_kg_s: f64) -> Self {
        let inlet_temp_k = 1_100.0;
        let gamma = 1.33_f64;
        let cp = 1_150.0_f64;
        let delta_t = shaft_power_kw * 1_000.0 / (mass_flow_kg_s * cp * 0.90);
        let pressure_ratio = (1.0 - delta_t / inlet_temp_k).powf(-gamma / (gamma - 1.0));
        Self {
            stage_type: TurbineStageType::PowerTurbine,
            blade_count: 100,
            inlet_temp_k,
            pressure_ratio,
            isentropic_efficiency: 0.90,
            cooling_air_fraction: 0.0,
            blade_material: TurbineBladeMaterial::Inconel718,
        }
    }

    pub fn outlet_temp_k(&self) -> f64 {
        let gamma = 1.33_f64;
        let exponent = (gamma - 1.0) / gamma;
        let t_ideal = self.inlet_temp_k / self.pressure_ratio.powf(exponent);
        self.inlet_temp_k - self.isentropic_efficiency * (self.inlet_temp_k - t_ideal)
    }
}
