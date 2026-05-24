#[derive(Debug, Clone)]
pub struct Ebd {
    pub max_rear_brake_force_pct: u8,
    pub load_sensing: bool,
    pub dynamic: bool,
}

impl Ebd {
    pub fn standard() -> Self {
        Self { max_rear_brake_force_pct: 40, load_sensing: false, dynamic: false }
    }

    pub fn dynamic() -> Self {
        Self { max_rear_brake_force_pct: 50, load_sensing: true, dynamic: true }
    }
}
