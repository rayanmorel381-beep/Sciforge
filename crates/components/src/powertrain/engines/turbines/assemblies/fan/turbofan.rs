#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbofanGen {
    LowBypass,
    HighBypass,
    Geared,
    UltraHighBypass,
}

#[derive(Debug, Clone)]
pub struct Turbofan {
    pub generation: TurbofanGen,
    pub thrust_kn: f64,
    pub bypass_ratio: f64,
    pub fan_diameter_m: f64,
    pub overall_pressure_ratio: f64,
    pub turbine_entry_temp_c: f64,
    pub sfc_kg_kn_h: f64,
}
