use super::turboprop::{Turboprop, TurbopropGearbox};

pub fn auxiliary(shaft_power_kw: f64, propeller_diameter_m: f64) -> Turboprop {
    Turboprop {
        shaft_power_kw,
        propeller_diameter_m,
        gearbox_type: TurbopropGearbox::DirectDrive,
        gearbox_ratio: 1.0,
        power_turbine_stages: 1,
        sfc_kg_kw_h: 0.31,
        flat_rating_alt_m: 1_500,
    }
}
