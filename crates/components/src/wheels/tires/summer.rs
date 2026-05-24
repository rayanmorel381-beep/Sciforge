#[derive(Debug, Clone)]
pub struct SummerTire {
    pub width_mm: u16,
    pub aspect_ratio: u8,
    pub rim_diameter_inch: u8,
    pub load_index: u8,
    pub speed_rating: char,
    pub run_flat: bool,
}

impl SummerTire {
    pub fn new(width_mm: u16, aspect_ratio: u8, rim_diameter_inch: u8, load_index: u8, speed_rating: char) -> Self {
        Self { width_mm, aspect_ratio, rim_diameter_inch, load_index, speed_rating, run_flat: false }
    }

    pub fn run_flat(width_mm: u16, aspect_ratio: u8, rim_diameter_inch: u8, load_index: u8, speed_rating: char) -> Self {
        Self { width_mm, aspect_ratio, rim_diameter_inch, load_index, speed_rating, run_flat: true }
    }
}
