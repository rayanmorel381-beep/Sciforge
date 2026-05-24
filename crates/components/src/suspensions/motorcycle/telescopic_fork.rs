#[derive(Debug, Clone)]
pub struct TelescopicFork {
    pub tube_diameter_mm: f64,
    pub travel_mm: f64,
    pub offset_mm: f64,
    pub cartridge: bool,
    pub inverted: bool,
}

impl TelescopicFork {
    pub fn conventional(tube_diameter_mm: f64, travel_mm: f64, offset_mm: f64) -> Self {
        Self { tube_diameter_mm, travel_mm, offset_mm, cartridge: false, inverted: false }
    }

    pub fn cartridge(tube_diameter_mm: f64, travel_mm: f64, offset_mm: f64) -> Self {
        Self { tube_diameter_mm, travel_mm, offset_mm, cartridge: true, inverted: false }
    }

    pub fn upside_down(tube_diameter_mm: f64, travel_mm: f64, offset_mm: f64) -> Self {
        Self { tube_diameter_mm, travel_mm, offset_mm, cartridge: true, inverted: true }
    }
}
