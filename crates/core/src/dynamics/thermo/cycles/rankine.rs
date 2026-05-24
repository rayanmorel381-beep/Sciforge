pub struct RankineCycle {
    pub h1_j_kg: f64,
    pub h2_j_kg: f64,
    pub h3_j_kg: f64,
    pub h4_j_kg: f64,
    pub eta_turbine: f64,
    pub eta_pump: f64,
}

impl RankineCycle {
    pub fn new(
        h1_j_kg: f64,
        h2_j_kg: f64,
        h3_j_kg: f64,
        h4_j_kg: f64,
        eta_turbine: f64,
        eta_pump: f64,
    ) -> Self {
        Self { h1_j_kg, h2_j_kg, h3_j_kg, h4_j_kg, eta_turbine, eta_pump }
    }

    pub fn work_turbine_j_kg(&self) -> f64 {
        (self.h1_j_kg - self.h2_j_kg) * self.eta_turbine
    }

    pub fn work_pump_j_kg(&self) -> f64 {
        (self.h4_j_kg - self.h3_j_kg) / self.eta_pump
    }

    pub fn net_work_j_kg(&self) -> f64 {
        self.work_turbine_j_kg() - self.work_pump_j_kg()
    }

    pub fn q_in_j_kg(&self) -> f64 {
        self.h1_j_kg - self.h4_j_kg
    }

    pub fn q_out_j_kg(&self) -> f64 {
        self.h2_j_kg - self.h3_j_kg
    }

    pub fn efficiency(&self) -> f64 {
        self.net_work_j_kg() / self.q_in_j_kg()
    }

    pub fn back_work_ratio(&self) -> f64 {
        self.work_pump_j_kg() / self.work_turbine_j_kg()
    }

    pub fn steam_rate_kg_j(&self) -> f64 {
        1.0 / self.net_work_j_kg()
    }
}
