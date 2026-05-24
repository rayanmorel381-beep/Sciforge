use crate::lubrications::Grease;

const MM2_S_TO_M2_S: f64 = 1.0e-6;

impl Grease {
    pub fn kinematic_viscosity_m2_s(&self) -> f64 {
        self.base_oil_viscosity_mm2_s * MM2_S_TO_M2_S
    }

    pub fn dynamic_viscosity_pa_s(&self) -> f64 {
        self.kinematic_viscosity_m2_s() * self.density_kg_m3
    }

    pub fn temperature_margin_below_dropping_k(&self, operating_k: f64) -> f64 {
        self.dropping_point_k - operating_k
    }

    pub fn temperature_span_k(&self) -> f64 {
        self.operating_temp_max_k - self.operating_temp_min_k
    }

    pub fn is_within_safe_dropping_margin(&self, operating_k: f64, margin_k: f64) -> bool {
        self.temperature_margin_below_dropping_k(operating_k) >= margin_k
    }

    pub fn load_safety_factor(&self, applied_load_n: f64) -> f64 {
        self.weld_point_n / applied_load_n
    }

    pub fn friction_force_n(&self, normal_load_n: f64) -> f64 {
        self.friction_coefficient * normal_load_n
    }

    pub fn power_loss_w(&self, normal_load_n: f64, sliding_velocity_m_s: f64) -> f64 {
        self.friction_force_n(normal_load_n) * sliding_velocity_m_s
    }

    pub fn walther_kinematic_viscosity_mm2_s(
        &self,
        reference_temp_k: f64,
        target_temp_k: f64,
        slope_b: f64,
    ) -> f64 {
        let log_log_ref = ((self.base_oil_viscosity_mm2_s + 0.7).ln()).ln();
        let a = log_log_ref + slope_b * reference_temp_k.ln();
        let log_log_target = a - slope_b * target_temp_k.ln();
        log_log_target.exp().exp() - 0.7
    }
}
