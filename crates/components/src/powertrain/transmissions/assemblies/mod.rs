pub mod automatics;
pub mod cvts;
pub mod robotized;
pub mod manuals;
pub mod sequentials;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GearboxFailureMode {
    GearToothPittingFatigue,
    GearToothBendingBroken,
    BearingSeized,
    SynchronizerWorn,
    DogRingChipped,
    ShiftDrumWear,
    ShiftForkBent,
    OilStarvation,
    OilOverheat,
    OverTorqueDamage,
    ClutchPackBurnt,
    TorqueConverterOverheat,
    BeltSlipExcessive,
    RollerSkid,
}

impl GearboxFailureMode {
    pub fn power_loss_fraction(self) -> f64 {
        match self {
            GearboxFailureMode::GearToothPittingFatigue => 0.05,
            GearboxFailureMode::GearToothBendingBroken => 1.0,
            GearboxFailureMode::BearingSeized => 1.0,
            GearboxFailureMode::SynchronizerWorn => 0.0,
            GearboxFailureMode::DogRingChipped => 0.10,
            GearboxFailureMode::ShiftDrumWear => 0.0,
            GearboxFailureMode::ShiftForkBent => 0.5,
            GearboxFailureMode::OilStarvation => 0.30,
            GearboxFailureMode::OilOverheat => 0.15,
            GearboxFailureMode::OverTorqueDamage => 1.0,
            GearboxFailureMode::ClutchPackBurnt => 1.0,
            GearboxFailureMode::TorqueConverterOverheat => 0.40,
            GearboxFailureMode::BeltSlipExcessive => 0.50,
            GearboxFailureMode::RollerSkid => 0.35,
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            GearboxFailureMode::GearToothBendingBroken
                | GearboxFailureMode::BearingSeized
                | GearboxFailureMode::OverTorqueDamage
                | GearboxFailureMode::ClutchPackBurnt
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GearboxOperatingPoint {
    pub gear: u8,
    pub ratio: f64,
    pub rpm_in: f64,
    pub rpm_out: f64,
    pub torque_in_nm: f64,
    pub torque_out_nm: f64,
    pub mesh_eff: f64,
    pub churn_loss_kw: f64,
    pub bearing_loss_kw: f64,
    pub effective_eff: f64,
    pub power_in_kw: f64,
    pub power_out_kw: f64,
    pub hertz_pressure_pa: f64,
    pub oil_temp_k: f64,
}
