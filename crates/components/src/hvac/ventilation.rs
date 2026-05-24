#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VentilationZone {
    SingleZone,
    DualZone,
    TriZone,
    QuadZone,
}

#[derive(Debug, Clone)]
pub struct Ventilation {
    pub zone: VentilationZone,
    pub air_flow_m3_h: f64,
    pub filtration: FiltrationGrade,
    pub auto: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiltrationGrade {
    Standard,
    Hepa,
    Activated,
}

impl Ventilation {
    pub fn single(air_flow_m3_h: f64) -> Self {
        Self { zone: VentilationZone::SingleZone, air_flow_m3_h, filtration: FiltrationGrade::Standard, auto: false }
    }

    pub fn dual(air_flow_m3_h: f64) -> Self {
        Self { zone: VentilationZone::DualZone, air_flow_m3_h, filtration: FiltrationGrade::Standard, auto: true }
    }

    pub fn quad(air_flow_m3_h: f64) -> Self {
        Self { zone: VentilationZone::QuadZone, air_flow_m3_h, filtration: FiltrationGrade::Hepa, auto: true }
    }
}
