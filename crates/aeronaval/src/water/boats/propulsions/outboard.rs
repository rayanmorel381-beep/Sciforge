#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarineConfig {
    OutboardPetrol,
}

#[derive(Debug, Clone)]
pub struct MarineEngine {
    pub displacement_cc: u32,
    pub power_kw: f64,
    pub torque_nm: f64,
    pub rpm_max: f64,
    pub config: MarineConfig,
}

pub fn outboard_small() -> MarineEngine {
    MarineEngine {
        displacement_cc: 339,
        power_kw: 7.4,
        torque_nm: 18.0,
        rpm_max: 5_500.0,
        config: MarineConfig::OutboardPetrol,
    }
}

pub fn outboard_mid() -> MarineEngine {
    MarineEngine {
        displacement_cc: 996,
        power_kw: 73.5,
        torque_nm: 143.0,
        rpm_max: 6_000.0,
        config: MarineConfig::OutboardPetrol,
    }
}

pub fn all() -> Vec<MarineEngine> {
    vec![outboard_small(), outboard_mid()]
}
