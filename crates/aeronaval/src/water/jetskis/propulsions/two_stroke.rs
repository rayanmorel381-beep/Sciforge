#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JetskiCycle {
    TwoStroke,
}

#[derive(Debug, Clone)]
pub struct JetskiEngine {
    pub displacement_cc: u32,
    pub power_kw: f64,
    pub torque_nm: f64,
    pub rpm_max: f64,
    pub cylinders: u8,
    pub cycle: JetskiCycle,
    pub supercharged: bool,
}

pub fn two_stroke_classic() -> JetskiEngine {
    JetskiEngine {
        displacement_cc: 701,
        power_kw: 48.5,
        torque_nm: 62.0,
        rpm_max: 6_750.0,
        cylinders: 2,
        cycle: JetskiCycle::TwoStroke,
        supercharged: false,
    }
}

pub fn all() -> Vec<JetskiEngine> {
    vec![two_stroke_classic()]
}
