#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GliderDrive {
    LinearActuator,
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

pub fn shallow_survey() -> UnderwaterGlider {
    UnderwaterGlider {
        power_w: 6.0,
        thrust_n: 3.0,
        speed_ms: 0.4,
        depth_m: 200.0,
        battery_wh: 800.0,
        drive: GliderDrive::LinearActuator,
    }
}

pub fn deep_survey() -> UnderwaterGlider {
    UnderwaterGlider {
        power_w: 12.0,
        thrust_n: 5.0,
        speed_ms: 0.5,
        depth_m: 1_000.0,
        battery_wh: 2_400.0,
        drive: GliderDrive::LinearActuator,
    }
}

pub fn all() -> Vec<UnderwaterGlider> {
    vec![shallow_survey(), deep_survey()]
}
