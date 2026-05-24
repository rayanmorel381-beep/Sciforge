#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaterSource {
    CoolantCore,
    Electric,
    HeatPump,
    Combustion,
}

#[derive(Debug, Clone)]
pub struct Heater {
    pub source: HeaterSource,
    pub output_kw: f64,
    pub fast_heating: bool,
}

impl Heater {
    pub fn coolant_core(output_kw: f64) -> Self {
        Self { source: HeaterSource::CoolantCore, output_kw, fast_heating: false }
    }

    pub fn electric(output_kw: f64) -> Self {
        Self { source: HeaterSource::Electric, output_kw, fast_heating: true }
    }

    pub fn heat_pump(output_kw: f64) -> Self {
        Self { source: HeaterSource::HeatPump, output_kw, fast_heating: false }
    }

    pub fn combustion(output_kw: f64) -> Self {
        Self { source: HeaterSource::Combustion, output_kw, fast_heating: true }
    }
}
