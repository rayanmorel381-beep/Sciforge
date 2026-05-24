#[derive(Debug, Clone)]
pub struct OpenDifferential {
    pub ring_gear_diameter_mm: f64,
    pub final_drive_ratio: f64,
    pub drive_pinion_teeth: u16,
    pub ring_gear_teeth: u16,
    pub spider_gear_count: u8,
    pub spider_gear_teeth: u16,
    pub side_gear_teeth: u16,
}

impl OpenDifferential {
    pub fn new(
        ring_gear_diameter_mm: f64,
        final_drive_ratio: f64,
        drive_pinion_teeth: u16,
        ring_gear_teeth: u16,
        spider_gear_count: u8,
        spider_gear_teeth: u16,
        side_gear_teeth: u16,
    ) -> Self {
        Self {
            ring_gear_diameter_mm,
            final_drive_ratio,
            drive_pinion_teeth,
            ring_gear_teeth,
            spider_gear_count,
            spider_gear_teeth,
            side_gear_teeth,
        }
    }
}
