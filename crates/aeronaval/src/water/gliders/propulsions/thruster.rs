#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GliderDrive {
    BrushlessMotor,
}

#[derive(Debug, Clone)]
pub struct UnderwaterGlider {
    pub power_w: f64,
    pub thrust_n: f64,
    pub speed_ms: f64,
    pub depth_m: f64,
    pub battery_wh: f64,
    pub drive: GliderDrive,
}

pub fn auv_torpedo() -> UnderwaterGlider {
    UnderwaterGlider {
        power_w: 600.0,
        thrust_n: 120.0,
        speed_ms: 2.5,
        depth_m: 500.0,
        battery_wh: 3_000.0,
        drive: GliderDrive::BrushlessMotor,
    }
}

pub fn all() -> Vec<UnderwaterGlider> {
    vec![auv_torpedo()]
}
