pub mod camshafts;
pub mod cc;
pub mod coolings;
pub mod crankshafts;
pub mod cylinders;
pub mod exhausts;
pub mod flywheels;
pub mod fuel_systems;
pub mod intakes;
pub mod pistons;
pub mod sounds;
pub mod valves;

pub const CYLINDERS: u8 = 12;

#[derive(Debug, Clone, Copy)]
pub struct Displacement {
    pub cc: u32,
    pub bore_mm: f64,
    pub stroke_mm: f64,
}

impl Displacement {
    pub const fn liters(&self) -> f64 {
        self.cc as f64 / 1000.0
    }
}
