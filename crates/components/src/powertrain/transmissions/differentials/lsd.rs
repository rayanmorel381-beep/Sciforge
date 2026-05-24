#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsdType {
    Mechanical,
    Viscous,
    Electronic,
    Hydraulic,
}

#[derive(Debug, Clone)]
pub struct LimitedSlipDifferential {
    pub lsd_type: LsdType,
    pub ring_gear_diameter_mm: f64,
    pub final_drive_ratio: f64,
    pub lock_percentage: u8,
    pub preload_nm: f64,
    pub drive_pinion_teeth: u16,
    pub ring_gear_teeth: u16,
    pub spider_gear_count: u8,
    pub spider_gear_teeth: u16,
    pub side_gear_teeth: u16,
}

impl LimitedSlipDifferential {
    pub fn mechanical(
        ring_gear_diameter_mm: f64,
        final_drive_ratio: f64,
        drive_pinion_teeth: u16,
        ring_gear_teeth: u16,
        spider_gear_count: u8,
        spider_gear_teeth: u16,
        side_gear_teeth: u16,
    ) -> Self {
        Self {
            lsd_type: LsdType::Mechanical,
            ring_gear_diameter_mm,
            final_drive_ratio,
            lock_percentage: 45,
            preload_nm: 60.0,
            drive_pinion_teeth,
            ring_gear_teeth,
            spider_gear_count,
            spider_gear_teeth,
            side_gear_teeth,
        }
    }

    pub fn viscous(
        ring_gear_diameter_mm: f64,
        final_drive_ratio: f64,
        drive_pinion_teeth: u16,
        ring_gear_teeth: u16,
        spider_gear_count: u8,
        spider_gear_teeth: u16,
        side_gear_teeth: u16,
    ) -> Self {
        Self {
            lsd_type: LsdType::Viscous,
            ring_gear_diameter_mm,
            final_drive_ratio,
            lock_percentage: 30,
            preload_nm: 0.0,
            drive_pinion_teeth,
            ring_gear_teeth,
            spider_gear_count,
            spider_gear_teeth,
            side_gear_teeth,
        }
    }

    pub fn electronic(
        ring_gear_diameter_mm: f64,
        final_drive_ratio: f64,
        drive_pinion_teeth: u16,
        ring_gear_teeth: u16,
        spider_gear_count: u8,
        spider_gear_teeth: u16,
        side_gear_teeth: u16,
    ) -> Self {
        Self {
            lsd_type: LsdType::Electronic,
            ring_gear_diameter_mm,
            final_drive_ratio,
            lock_percentage: 100,
            preload_nm: 0.0,
            drive_pinion_teeth,
            ring_gear_teeth,
            spider_gear_count,
            spider_gear_teeth,
            side_gear_teeth,
        }
    }
}
