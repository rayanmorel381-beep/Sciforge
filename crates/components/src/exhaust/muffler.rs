#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MufflerType {
    Absorption,
    Reactive,
    Performance,
    Straight,
    ActiveValve,
}

#[derive(Debug, Clone)]
pub struct Muffler {
    pub muffler_type: MufflerType,
    pub inlet_diameter_mm: f64,
    pub outlet_diameter_mm: f64,
    pub volume_l: f64,
}

impl Muffler {
    pub fn standard(inlet_diameter_mm: f64) -> Self {
        Self { muffler_type: MufflerType::Reactive, inlet_diameter_mm, outlet_diameter_mm: inlet_diameter_mm, volume_l: 8.0 }
    }

    pub fn performance(inlet_diameter_mm: f64, outlet_diameter_mm: f64) -> Self {
        Self { muffler_type: MufflerType::Absorption, inlet_diameter_mm, outlet_diameter_mm, volume_l: 6.0 }
    }

    pub fn active_valve(inlet_diameter_mm: f64, outlet_diameter_mm: f64) -> Self {
        Self { muffler_type: MufflerType::ActiveValve, inlet_diameter_mm, outlet_diameter_mm, volume_l: 7.0 }
    }
}
