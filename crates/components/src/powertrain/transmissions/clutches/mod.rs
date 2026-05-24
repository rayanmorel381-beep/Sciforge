pub mod dry;
pub mod wet;

pub use dry::DryCutch;
pub use wet::WetClutch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DualClutchKind {
    Dry,
    Wet,
}

#[derive(Debug, Clone)]
pub struct DualClutch {
    pub kind: DualClutchKind,
    pub max_torque_nm: f64,
}

impl DualClutch {
    pub fn dry(max_torque_nm: f64) -> Self {
        Self { kind: DualClutchKind::Dry, max_torque_nm }
    }

    pub fn wet(max_torque_nm: f64) -> Self {
        Self { kind: DualClutchKind::Wet, max_torque_nm }
    }
}

#[derive(Debug, Clone)]
pub enum Clutch {
    Dry(DryCutch),
    Wet(WetClutch),
    Dual(DualClutch),
}

impl From<DryCutch> for Clutch {
    fn from(c: DryCutch) -> Self { Clutch::Dry(c) }
}

impl From<WetClutch> for Clutch {
    fn from(c: WetClutch) -> Self { Clutch::Wet(c) }
}

impl From<DualClutch> for Clutch {
    fn from(c: DualClutch) -> Self { Clutch::Dual(c) }
}
