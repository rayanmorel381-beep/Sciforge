use sciforge_core::materials::nickels::inconel::INCONEL_718;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BearingType {
    RollerTapered,
    BallAngularContact,
    ThrustBall,
    JournalDamper,
}

#[derive(Debug, Clone)]
pub struct MainBearing {
    pub bearing_type: BearingType,
    pub bore_mm: f64,
    pub dynamic_load_kn: f64,
    pub max_dn: f64,
    pub material_density_kg_m3: f64,
    pub tbo_hours: u32,
}

impl MainBearing {
    pub fn roller_tapered(bore_mm: f64, dynamic_load_kn: f64) -> Self {
        Self {
            bearing_type: BearingType::RollerTapered,
            bore_mm,
            dynamic_load_kn,
            max_dn: 1_800_000.0,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            tbo_hours: 20_000,
        }
    }

    pub fn ball_angular(bore_mm: f64, dynamic_load_kn: f64) -> Self {
        Self {
            bearing_type: BearingType::BallAngularContact,
            bore_mm,
            dynamic_load_kn,
            max_dn: 2_500_000.0,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            tbo_hours: 15_000,
        }
    }

    pub fn journal_damper(bore_mm: f64, dynamic_load_kn: f64) -> Self {
        Self {
            bearing_type: BearingType::JournalDamper,
            bore_mm,
            dynamic_load_kn,
            max_dn: 3_500_000.0,
            material_density_kg_m3: INCONEL_718.density_kg_m3,
            tbo_hours: 25_000,
        }
    }

    pub fn dn(&self, rpm: u32) -> f64 {
        self.bore_mm * rpm as f64
    }

    pub fn is_overspeed(&self, rpm: u32) -> bool {
        self.dn(rpm) > self.max_dn
    }
}
