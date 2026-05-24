#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarineConfig {
    InboardDiesel,
    SternDrive,
}

#[derive(Debug, Clone)]
pub struct MarineEngine {
    pub displacement_cc: u32,
    pub power_kw: f64,
    pub torque_nm: f64,
    pub rpm_max: f64,
    pub config: MarineConfig,
}

pub fn inboard_diesel_cruiser() -> MarineEngine {
    MarineEngine {
        displacement_cc: 5_652,
        power_kw: 186.0,
        torque_nm: 810.0,
        rpm_max: 3_800.0,
        config: MarineConfig::InboardDiesel,
    }
}

pub fn stern_drive() -> MarineEngine {
    MarineEngine {
        displacement_cc: 4_296,
        power_kw: 149.0,
        torque_nm: 390.0,
        rpm_max: 5_000.0,
        config: MarineConfig::SternDrive,
    }
}

pub fn all() -> Vec<MarineEngine> {
    vec![inboard_diesel_cruiser(), stern_drive()]
}
