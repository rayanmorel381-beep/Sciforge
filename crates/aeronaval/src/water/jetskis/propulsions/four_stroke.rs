#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JetskiCycle {
    FourStroke,
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

pub fn four_stroke_sport() -> JetskiEngine {
    JetskiEngine {
        displacement_cc: 1_052,
        power_kw: 60.0,
        torque_nm: 90.0,
        rpm_max: 7_500.0,
        cylinders: 3,
        cycle: JetskiCycle::FourStroke,
        supercharged: false,
    }
}

pub fn four_stroke_supercharged() -> JetskiEngine {
    JetskiEngine {
        displacement_cc: 1_503,
        power_kw: 191.0,
        torque_nm: 210.0,
        rpm_max: 8_000.0,
        cylinders: 4,
        cycle: JetskiCycle::FourStroke,
        supercharged: true,
    }
}

pub fn all() -> Vec<JetskiEngine> {
    vec![four_stroke_sport(), four_stroke_supercharged()]
}
