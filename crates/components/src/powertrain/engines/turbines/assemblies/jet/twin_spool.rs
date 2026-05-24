use super::turbojet::{Turbojet, TurbojetType};

pub fn standard(thrust_kn: f64, pressure_ratio: f64) -> Turbojet {
    Turbojet {
        jet_type: TurbojetType::TwinSpool,
        thrust_kn,
        thrust_afterburner_kn: None,
        pressure_ratio,
        turbine_inlet_temp_c: 1_250.0,
        mass_flow_kg_s: thrust_kn / 1.2,
        sfc_kg_kn_h: 0.085,
    }
}

pub fn high_performance(thrust_kn: f64, pressure_ratio: f64) -> Turbojet {
    Turbojet {
        jet_type: TurbojetType::TwinSpool,
        thrust_kn,
        thrust_afterburner_kn: None,
        pressure_ratio,
        turbine_inlet_temp_c: 1_350.0,
        mass_flow_kg_s: thrust_kn / 1.25,
        sfc_kg_kn_h: 0.082,
    }
}
