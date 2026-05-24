#[derive(Debug, Clone)]
pub struct WinterTire {
    pub width_mm: u16,
    pub aspect_ratio: u8,
    pub rim_diameter_inch: u8,
    pub load_index: u8,
    pub speed_rating: char,
    pub studded: bool,
}

impl WinterTire {
    pub fn studless(width_mm: u16, aspect_ratio: u8, rim_diameter_inch: u8, load_index: u8, speed_rating: char) -> Self {
        Self { width_mm, aspect_ratio, rim_diameter_inch, load_index, speed_rating, studded: false }
    }

    pub fn studded(width_mm: u16, aspect_ratio: u8, rim_diameter_inch: u8, load_index: u8, speed_rating: char) -> Self {
        Self { width_mm, aspect_ratio, rim_diameter_inch, load_index, speed_rating, studded: true }
    }
}
