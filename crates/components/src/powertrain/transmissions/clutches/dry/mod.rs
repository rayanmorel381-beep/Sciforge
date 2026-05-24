pub mod simple;
pub mod dual;
pub mod multi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DryClutchKind {
    Single,
    Twin,
}

#[derive(Debug, Clone)]
pub struct DryCutch {
    pub kind: DryClutchKind,
    pub disc_diameter_mm: f64,
    pub max_torque_nm: f64,
}

impl DryCutch {
    pub fn single(disc_diameter_mm: f64, max_torque_nm: f64) -> Self {
        Self { kind: DryClutchKind::Single, disc_diameter_mm, max_torque_nm }
    }

    pub fn twin(disc_diameter_mm: f64, max_torque_nm: f64) -> Self {
        Self { kind: DryClutchKind::Twin, disc_diameter_mm, max_torque_nm }
    }
}
