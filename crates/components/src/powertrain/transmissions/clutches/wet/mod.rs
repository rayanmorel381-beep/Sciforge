pub mod simple;
pub mod dual;
pub mod multi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WetClutchKind {
    Motorcycle,
    Atv,
}

#[derive(Debug, Clone)]
pub struct WetClutch {
    pub kind: WetClutchKind,
    pub disc_count: u8,
    pub disc_diameter_mm: f64,
    pub max_torque_nm: f64,
}

impl WetClutch {
    pub fn motorcycle(disc_count: u8, disc_diameter_mm: f64, max_torque_nm: f64) -> Self {
        Self { kind: WetClutchKind::Motorcycle, disc_count, disc_diameter_mm, max_torque_nm }
    }

    pub fn atv(disc_count: u8, disc_diameter_mm: f64, max_torque_nm: f64) -> Self {
        Self { kind: WetClutchKind::Atv, disc_count, disc_diameter_mm, max_torque_nm }
    }
}
