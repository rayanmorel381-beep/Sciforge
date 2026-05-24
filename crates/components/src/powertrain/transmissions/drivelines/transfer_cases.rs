use crate::powertrain::transmissions::differentials::Differential;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferCaseKind {
    PartTimeChain,
    PartTimeGear,
    FullTimeChain,
    FullTimeGear,
    Active,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LowRange {
    None,
    Single { ratio_x10: u16 },
    Dual { high_x10: u16, low_x10: u16 },
}

#[derive(Debug, Clone)]
pub struct TransferCase {
    pub kind: TransferCaseKind,
    pub low_range: LowRange,
    pub front_torque_split_pct: f64,
    pub rear_torque_split_pct: f64,
    pub center_differential: Option<Differential>,
    pub max_input_torque_nm: f64,
    pub length_mm: f64,
    pub mass_kg: f64,
}

pub fn assemble(
    kind: TransferCaseKind,
    low_range: LowRange,
    front_split_pct: f64,
    center_differential: Option<Differential>,
    max_input_torque_nm: f64,
) -> TransferCase {
    let rear_split_pct = (100.0 - front_split_pct).clamp(0.0, 100.0);
    let length_mm = match kind {
        TransferCaseKind::PartTimeChain | TransferCaseKind::FullTimeChain => 220.0,
        TransferCaseKind::PartTimeGear | TransferCaseKind::FullTimeGear => 280.0,
        TransferCaseKind::Active => 260.0,
    };
    let base_mass = match kind {
        TransferCaseKind::PartTimeChain => 28.0,
        TransferCaseKind::FullTimeChain => 35.0,
        TransferCaseKind::PartTimeGear => 42.0,
        TransferCaseKind::FullTimeGear => 48.0,
        TransferCaseKind::Active => 45.0,
    };
    let mass_kg = base_mass + (max_input_torque_nm / 100.0).sqrt() * 4.0;
    TransferCase {
        kind,
        low_range,
        front_torque_split_pct: front_split_pct,
        rear_torque_split_pct: rear_split_pct,
        center_differential,
        max_input_torque_nm,
        length_mm,
        mass_kg: (mass_kg * 100.0).round() / 100.0,
    }
}
