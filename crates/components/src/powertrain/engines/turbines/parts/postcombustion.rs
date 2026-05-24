#[derive(Debug, Clone)]
pub struct Postcombustion {
    pub fuel_flow_max_kg_h: f64,
    pub max_egt_k: f64,
    pub pressure_loss_fraction: f64,
    pub thrust_augmentation_ratio: f64,
    pub spray_bar_count: u8,
}

impl Postcombustion {
    pub fn single_stage(base_thrust_kn: f64) -> Self {
        Self {
            fuel_flow_max_kg_h: base_thrust_kn * 420.0,
            max_egt_k: 2_000.0,
            pressure_loss_fraction: 0.025,
            thrust_augmentation_ratio: 1.50,
            spray_bar_count: 8,
        }
    }

    pub fn advanced(base_thrust_kn: f64, augmentation_ratio: f64) -> Self {
        Self {
            fuel_flow_max_kg_h: base_thrust_kn * 480.0,
            max_egt_k: 2_150.0,
            pressure_loss_fraction: 0.020,
            thrust_augmentation_ratio: augmentation_ratio,
            spray_bar_count: 12,
        }
    }

    pub fn wet_thrust_kn(&self, dry_thrust_kn: f64) -> f64 {
        dry_thrust_kn * self.thrust_augmentation_ratio
    }
}
