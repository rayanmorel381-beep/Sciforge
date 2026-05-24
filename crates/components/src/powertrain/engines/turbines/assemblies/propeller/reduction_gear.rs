use super::turboprop::{Turboprop, TurbopropGearbox};

pub fn standard(shaft_power_kw: f64, propeller_diameter_m: f64, gearbox_ratio: f64) -> Turboprop {
    Turboprop {
        shaft_power_kw,
        propeller_diameter_m,
        gearbox_type: TurbopropGearbox::ReductionGear,
        gearbox_ratio,
        power_turbine_stages: 1,
        sfc_kg_kw_h: 0.28,
        flat_rating_alt_m: 5_000,
    }
}

pub fn high_altitude(shaft_power_kw: f64, propeller_diameter_m: f64, gearbox_ratio: f64) -> Turboprop {
    Turboprop {
        shaft_power_kw,
        propeller_diameter_m,
        gearbox_type: TurbopropGearbox::ReductionGear,
        gearbox_ratio,
        power_turbine_stages: 2,
        sfc_kg_kw_h: 0.26,
        flat_rating_alt_m: 9_000,
    }
}
