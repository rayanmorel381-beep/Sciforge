use sciforge_core::materials::Material;
use sciforge_core::materials::nickels::inconel::INCONEL_713C;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[derive(Debug, Clone)]
pub struct Turbocharger {
    pub compressor_inducer_mm: f64,
    pub turbine_exducer_mm: f64,
    pub max_boost_bar: f64,
}

impl Turbocharger {
    pub fn compressor_wheel_material(&self) -> &'static Material {
        &TI6AL4V_GR5
    }

    pub fn turbine_wheel_material(&self) -> &'static Material {
        &INCONEL_713C
    }

    pub fn slip_factor(&self) -> f64 {
        let d_ratio = self.compressor_inducer_mm / self.turbine_exducer_mm.max(1.0);
        (1.0 - 0.63 * std::f64::consts::PI / 12.0 * d_ratio).clamp(0.78, 0.92)
    }

    pub fn isentropic_turbine_efficiency(&self) -> f64 {
        let d = self.turbine_exducer_mm;
        (0.62 + 0.10 * ((d - 40.0) / 40.0).tanh()).clamp(0.55, 0.78)
    }

    pub fn design_expansion_ratio(&self) -> f64 {
        (self.max_boost_bar + 1.0).max(1.2)
    }
}

impl Default for Turbocharger {
    fn default() -> Self {
        Self {
            compressor_inducer_mm: 46.0,
            turbine_exducer_mm: 49.0,
            max_boost_bar: 1.2,
        }
    }
}
