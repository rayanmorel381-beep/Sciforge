#[derive(Debug, Clone)]
pub struct OffRoadTire {
    pub width_mm: u16,
    pub aspect_ratio: u8,
    pub rim_diameter_inch: u8,
    pub load_index: u8,
    pub tread_depth_mm: f64,
    pub mud_terrain: bool,
}

impl OffRoadTire {
    pub fn all_terrain(width_mm: u16, aspect_ratio: u8, rim_diameter_inch: u8, load_index: u8) -> Self {
        Self { width_mm, aspect_ratio, rim_diameter_inch, load_index, tread_depth_mm: 10.0, mud_terrain: false }
    }

    pub fn mud_terrain(width_mm: u16, aspect_ratio: u8, rim_diameter_inch: u8, load_index: u8) -> Self {
        Self { width_mm, aspect_ratio, rim_diameter_inch, load_index, tread_depth_mm: 14.0, mud_terrain: true }
    }
}
