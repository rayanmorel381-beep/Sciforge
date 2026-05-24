#[derive(Debug, Clone)]
pub struct DieselParticulateFilter {
    pub volume_l: f64,
    pub filtration_efficiency_pct: f64,
    pub active_regeneration: bool,
}

impl DieselParticulateFilter {
    pub fn passive(volume_l: f64) -> Self {
        Self { volume_l, filtration_efficiency_pct: 99.9, active_regeneration: false }
    }

    pub fn active(volume_l: f64) -> Self {
        Self { volume_l, filtration_efficiency_pct: 99.9, active_regeneration: true }
    }
}
