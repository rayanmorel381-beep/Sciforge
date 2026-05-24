use super::turbofan::{Turbofan, TurbofanGen};

pub fn next_gen(thrust_kn: f64, bypass_ratio: f64, fan_diameter_m: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::Geared,
        thrust_kn,
        bypass_ratio,
        fan_diameter_m,
        overall_pressure_ratio: 50.0,
        turbine_entry_temp_c: 1_700.0,
        sfc_kg_kn_h: 0.046,
    }
}

pub fn regional(thrust_kn: f64, bypass_ratio: f64, fan_diameter_m: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::Geared,
        thrust_kn,
        bypass_ratio,
        fan_diameter_m,
        overall_pressure_ratio: 45.0,
        turbine_entry_temp_c: 1_650.0,
        sfc_kg_kn_h: 0.049,
    }
}
