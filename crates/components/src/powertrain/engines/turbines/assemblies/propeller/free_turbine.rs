use super::turboprop::{Turboprop, TurbopropGearbox};

pub fn twin_turbine(shaft_power_kw: f64, propeller_diameter_m: f64) -> Turboprop {
    Turboprop {
        shaft_power_kw,
        propeller_diameter_m,
        gearbox_type: TurbopropGearbox::FreeWheeling,
        gearbox_ratio: 14.5,
        power_turbine_stages: 2,
        sfc_kg_kw_h: 0.27,
        flat_rating_alt_m: 3_000,
    }
}

pub fn single_turbine(shaft_power_kw: f64, propeller_diameter_m: f64) -> Turboprop {
    Turboprop {
        shaft_power_kw,
        propeller_diameter_m,
        gearbox_type: TurbopropGearbox::FreeWheeling,
        gearbox_ratio: 12.0,
        power_turbine_stages: 1,
        sfc_kg_kw_h: 0.29,
        flat_rating_alt_m: 2_000,
    }
}
