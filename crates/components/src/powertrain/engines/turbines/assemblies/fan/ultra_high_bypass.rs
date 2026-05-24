use super::turbofan::{Turbofan, TurbofanGen};

pub fn open_rotor(thrust_kn: f64, bypass_ratio: f64, fan_diameter_m: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::UltraHighBypass,
        thrust_kn,
        bypass_ratio,
        fan_diameter_m,
        overall_pressure_ratio: 60.0,
        turbine_entry_temp_c: 1_750.0,
        sfc_kg_kn_h: 0.038,
    }
}

pub fn ducted(thrust_kn: f64, bypass_ratio: f64, fan_diameter_m: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::UltraHighBypass,
        thrust_kn,
        bypass_ratio,
        fan_diameter_m,
        overall_pressure_ratio: 55.0,
        turbine_entry_temp_c: 1_720.0,
        sfc_kg_kn_h: 0.040,
    }
}
