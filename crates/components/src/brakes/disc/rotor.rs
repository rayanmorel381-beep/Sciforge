#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotorMaterial {
    CastIron,
    CarbonCeramic,
    Drilled,
    Slotted,
    DrilledAndSlotted,
}

#[derive(Debug, Clone)]
pub struct Rotor {
    pub material: RotorMaterial,
    pub diameter_mm: f64,
    pub thickness_mm: f64,
    pub vented: bool,
    pub weight_kg: f64,
}

impl Rotor {
    pub fn cast_iron(diameter_mm: f64, thickness_mm: f64, vented: bool) -> Self {
        let weight_kg = (diameter_mm / 1000.0).powi(2) * thickness_mm / 1000.0 * 7200.0;
        Self { material: RotorMaterial::CastIron, diameter_mm, thickness_mm, vented, weight_kg }
    }

    pub fn carbon_ceramic(diameter_mm: f64, thickness_mm: f64) -> Self {
        let weight_kg = (diameter_mm / 1000.0).powi(2) * thickness_mm / 1000.0 * 2600.0;
        Self { material: RotorMaterial::CarbonCeramic, diameter_mm, thickness_mm, vented: true, weight_kg }
    }

    pub fn drilled(diameter_mm: f64, thickness_mm: f64) -> Self {
        let weight_kg = (diameter_mm / 1000.0).powi(2) * thickness_mm / 1000.0 * 6800.0;
        Self { material: RotorMaterial::Drilled, diameter_mm, thickness_mm, vented: true, weight_kg }
    }
}
