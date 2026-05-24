#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitterMaterial {
    CarbonFibre,
    Aluminum,
    Composite,
    ABS,
}

#[derive(Debug, Clone)]
pub struct Splitter {
    pub material: SplitterMaterial,
    pub depth_mm: f64,
    pub width_mm: f64,
    pub canards: u8,
}

impl Splitter {
    pub fn new(material: SplitterMaterial, depth_mm: f64, width_mm: f64) -> Self {
        Self { material, depth_mm, width_mm, canards: 0 }
    }

    pub fn with_canards(material: SplitterMaterial, depth_mm: f64, width_mm: f64, canards: u8) -> Self {
        Self { material, depth_mm, width_mm, canards }
    }
}
