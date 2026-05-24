#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotorConfig {
    SingleMain,
    Coaxial,
    Tandem,
    Intermeshing,
    Tiltrotor,
}

#[derive(Debug, Clone)]
pub struct RotorBlade {
    pub config: RotorConfig,
    pub diameter_m: f64,
    pub blade_count: u8,
    pub rpm_max: u16,
    pub fully_articulated: bool,
}

impl RotorBlade {
    pub fn single_main(diameter_m: f64, blade_count: u8) -> Self {
        Self { config: RotorConfig::SingleMain, diameter_m, blade_count, rpm_max: 450, fully_articulated: true }
    }

    pub fn coaxial(diameter_m: f64, blade_count: u8) -> Self {
        Self { config: RotorConfig::Coaxial, diameter_m, blade_count, rpm_max: 380, fully_articulated: true }
    }

    pub fn tandem(diameter_m: f64, blade_count: u8) -> Self {
        Self { config: RotorConfig::Tandem, diameter_m, blade_count, rpm_max: 250, fully_articulated: true }
    }

    pub fn tiltrotor(diameter_m: f64, blade_count: u8) -> Self {
        Self { config: RotorConfig::Tiltrotor, diameter_m, blade_count, rpm_max: 600, fully_articulated: false }
    }
}
