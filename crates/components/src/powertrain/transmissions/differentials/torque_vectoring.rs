#[derive(Debug, Clone)]
pub struct TorqueVectoring {
    pub ring_gear_diameter_mm: f64,
    pub final_drive_ratio: f64,
    pub max_vectoring_torque_nm: f64,
    pub response_time_ms: u32,
    pub electric_actuated: bool,
    pub drive_pinion_teeth: u16,
    pub ring_gear_teeth: u16,
    pub spider_gear_count: u8,
    pub spider_gear_teeth: u16,
    pub side_gear_teeth: u16,
}

impl TorqueVectoring {
    pub fn hydraulic(
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
            max_vectoring_torque_nm: 1500.0,
            response_time_ms: 100,
            electric_actuated: false,
            drive_pinion_teeth,
            ring_gear_teeth,
            spider_gear_count,
            spider_gear_teeth,
            side_gear_teeth,
        }
    }

    pub fn electric(
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
            max_vectoring_torque_nm: 2200.0,
            response_time_ms: 30,
            electric_actuated: true,
            drive_pinion_teeth,
            ring_gear_teeth,
            spider_gear_count,
            spider_gear_teeth,
            side_gear_teeth,
        }
    }
}
