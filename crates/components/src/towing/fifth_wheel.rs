#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FifthWheelType {
    Holland,
    Fontaine,
    Jost,
}

#[derive(Debug, Clone)]
pub struct FifthWheel {
    pub wheel_type: FifthWheelType,
    pub kingpin_mm: u8,
    pub max_load_kg: f64,
    pub sliding: bool,
}

impl FifthWheel {
    pub fn fixed(wheel_type: FifthWheelType, max_load_kg: f64) -> Self {
        Self { wheel_type, kingpin_mm: 90, max_load_kg, sliding: false }
    }

    pub fn sliding(wheel_type: FifthWheelType, max_load_kg: f64) -> Self {
        Self { wheel_type, kingpin_mm: 90, max_load_kg, sliding: true }
    }
}
