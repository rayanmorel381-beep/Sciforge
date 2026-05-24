use super::turbofan::{Turbofan, TurbofanGen};

pub fn standard(thrust_kn: f64, bypass_ratio: f64, fan_diameter_m: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::HighBypass,
        thrust_kn,
        bypass_ratio,
        fan_diameter_m,
        overall_pressure_ratio: 35.0,
        turbine_entry_temp_c: 1_550.0,
        sfc_kg_kn_h: 0.055,
    }
}

pub fn long_range(thrust_kn: f64, bypass_ratio: f64, fan_diameter_m: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::HighBypass,
        thrust_kn,
        bypass_ratio,
        fan_diameter_m,
        overall_pressure_ratio: 40.0,
        turbine_entry_temp_c: 1_600.0,
        sfc_kg_kn_h: 0.052,
    }
}
