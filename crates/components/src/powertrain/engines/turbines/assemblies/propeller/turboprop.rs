#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurbopropGearbox {
    FreeWheeling,
    DirectDrive,
    ReductionGear,
}

#[derive(Debug, Clone)]
pub struct Turboprop {
    pub shaft_power_kw: f64,
    pub propeller_diameter_m: f64,
    pub gearbox_type: TurbopropGearbox,
    pub gearbox_ratio: f64,
    pub power_turbine_stages: u8,
    pub sfc_kg_kw_h: f64,
    pub flat_rating_alt_m: u32,
}
