#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneLocation {
    Front,
    Rear,
    Side,
}

#[derive(Debug, Clone)]
pub struct CrumpleZone {
    pub location: ZoneLocation,
    pub energy_absorption_kj: f64,
    pub progressive: bool,
}

impl CrumpleZone {
    pub fn front(energy_absorption_kj: f64) -> Self {
        Self { location: ZoneLocation::Front, energy_absorption_kj, progressive: true }
    }

    pub fn rear(energy_absorption_kj: f64) -> Self {
        Self { location: ZoneLocation::Rear, energy_absorption_kj, progressive: true }
    }

    pub fn side(energy_absorption_kj: f64) -> Self {
        Self { location: ZoneLocation::Side, energy_absorption_kj, progressive: false }
    }
}
