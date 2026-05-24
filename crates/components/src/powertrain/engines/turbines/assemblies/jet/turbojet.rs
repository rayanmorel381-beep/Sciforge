#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbojetType {
    SingleSpool,
    TwinSpool,
    WithPostcombustion,
}

#[derive(Debug, Clone)]
pub struct Turbojet {
    pub jet_type: TurbojetType,
    pub thrust_kn: f64,
    pub thrust_afterburner_kn: Option<f64>,
    pub pressure_ratio: f64,
    pub turbine_inlet_temp_c: f64,
    pub mass_flow_kg_s: f64,
    pub sfc_kg_kn_h: f64,
}
