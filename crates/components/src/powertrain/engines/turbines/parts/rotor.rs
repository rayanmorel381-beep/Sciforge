use sciforge_core::materials::nickels::inconel::INCONEL_718;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotorStage {
    CompressorLp,
    CompressorHp,
    TurbineHp,
    TurbineLp,
    FanStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BladeRoot {
    Dovetail,
    Firtree,
    Integral,
}

#[derive(Debug, Clone)]
pub struct RotorDisc {
    pub stage: RotorStage,
    pub blade_count: u8,
    pub disc_diameter_m: f64,
    pub tip_clearance_mm: f64,
    pub blade_root: BladeRoot,
    pub max_rpm: u32,
    pub material_density_kg_m3: f64,
    pub max_stress_mpa: f64,
}

impl RotorDisc {
    pub fn compressor_lp(stage_count: u8, disc_diameter_m: f64) -> Self {
        Self {
            stage: RotorStage::CompressorLp,
            blade_count: 32 + stage_count * 4,
            disc_diameter_m,
            tip_clearance_mm: 0.8,
            blade_root: BladeRoot::Dovetail,
            max_rpm: 10_000,
            material_density_kg_m3: TI6AL4V_GR5.density_kg_m3,
            max_stress_mpa: 900.0,
        }
    }

    pub fn compressor_hp(stage_count: u8, disc_diameter_m: f64) -> Self {
        Self {
            stage: RotorStage::CompressorHp,
            blade_count: 60 + stage_count * 2,
            disc_diameter_m,
            tip_clearance_mm: 0.5,
            blade_root: BladeRoot::Firtree,
            max_rpm: 14_000,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            max_stress_mpa: 1_050.0,
        }
    }

    pub fn turbine_hp(disc_diameter_m: f64) -> Self {
        Self {
            stage: RotorStage::TurbineHp,
            blade_count: 80,
            disc_diameter_m,
            tip_clearance_mm: 0.3,
            blade_root: BladeRoot::Firtree,
            max_rpm: 14_000,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            max_stress_mpa: 950.0,
        }
    }

    pub fn turbine_lp(disc_diameter_m: f64) -> Self {
        Self {
            stage: RotorStage::TurbineLp,
            blade_count: 120,
            disc_diameter_m,
            tip_clearance_mm: 0.6,
            blade_root: BladeRoot::Firtree,
            max_rpm: 8_000,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            max_stress_mpa: 800.0,
        }
    }

    pub fn fan(disc_diameter_m: f64) -> Self {
        Self {
            stage: RotorStage::FanStage,
            blade_count: 18,
            disc_diameter_m,
            tip_clearance_mm: 1.5,
            blade_root: BladeRoot::Dovetail,
            max_rpm: 4_500,
            material_density_kg_m3: TI6AL4V_GR5.density_kg_m3,
            max_stress_mpa: 700.0,
        }
    }

    pub fn hoop_stress_mpa(&self, rpm: u32) -> f64 {
        let omega = rpm as f64 * 2.0 * std::f64::consts::PI / 60.0;
        let r = self.disc_diameter_m / 2.0;
        self.material_density_kg_m3 * omega.powi(2) * r.powi(2) / 1_000_000.0
    }

    pub fn is_burst_risk(&self, rpm: u32) -> bool {
        self.hoop_stress_mpa(rpm) > self.max_stress_mpa * 0.85
    }
}
