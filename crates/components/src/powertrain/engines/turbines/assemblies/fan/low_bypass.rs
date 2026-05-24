use super::turbofan::{Turbofan, TurbofanGen};

pub fn military(thrust_kn: f64, bypass_ratio: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::LowBypass,
        thrust_kn,
        bypass_ratio,
        fan_diameter_m: 0.75,
        overall_pressure_ratio: 25.0,
        turbine_entry_temp_c: 1_400.0,
        sfc_kg_kn_h: 0.08,
    }
}

pub fn afterburning(dry_kn: f64, bypass_ratio: f64) -> Turbofan {
    Turbofan {
        generation: TurbofanGen::LowBypass,
        thrust_kn: dry_kn,
        bypass_ratio,
        fan_diameter_m: 0.88,
        overall_pressure_ratio: 28.0,
        turbine_entry_temp_c: 1_500.0,
        sfc_kg_kn_h: 0.19,
    }
}
