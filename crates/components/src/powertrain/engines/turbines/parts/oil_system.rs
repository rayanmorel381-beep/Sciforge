#[derive(Debug, Clone)]
pub struct OilSystem {
    pub sump_capacity_l: f64,
    pub pump_pressure_bar: f64,
    pub consumption_ml_h: f64,
    pub filter_micron: u8,
    pub max_temp_k: f64,
    pub scavenge_ratio: f64,
}

impl OilSystem {
    pub fn turbofan(sump_capacity_l: f64) -> Self {
        Self {
            sump_capacity_l,
            pump_pressure_bar: 6.5,
            consumption_ml_h: 90.0,
            filter_micron: 15,
            max_temp_k: 478.15,
            scavenge_ratio: 3.0,
        }
    }

    pub fn turbojet(sump_capacity_l: f64) -> Self {
        Self {
            sump_capacity_l,
            pump_pressure_bar: 8.0,
            consumption_ml_h: 120.0,
            filter_micron: 15,
            max_temp_k: 483.15,
            scavenge_ratio: 3.5,
        }
    }

    pub fn turboprop(sump_capacity_l: f64) -> Self {
        Self {
            sump_capacity_l,
            pump_pressure_bar: 5.5,
            consumption_ml_h: 60.0,
            filter_micron: 20,
            max_temp_k: 473.15,
            scavenge_ratio: 2.5,
        }
    }

    pub fn hours_to_dry(&self) -> f64 {
        self.sump_capacity_l * 1_000.0 / self.consumption_ml_h
    }
}
