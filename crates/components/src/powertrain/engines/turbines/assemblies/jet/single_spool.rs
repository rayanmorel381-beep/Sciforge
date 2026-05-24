use super::turbojet::{Turbojet, TurbojetType};

pub fn basic(thrust_kn: f64, pressure_ratio: f64) -> Turbojet {
    Turbojet {
        jet_type: TurbojetType::SingleSpool,
        thrust_kn,
        thrust_afterburner_kn: None,
        pressure_ratio,
        turbine_inlet_temp_c: 1_100.0,
        mass_flow_kg_s: thrust_kn / 1.1,
        sfc_kg_kn_h: 0.095,
    }
}

pub fn high_altitude(thrust_kn: f64, pressure_ratio: f64) -> Turbojet {
    Turbojet {
        jet_type: TurbojetType::SingleSpool,
        thrust_kn,
        thrust_afterburner_kn: None,
        pressure_ratio: pressure_ratio * 1.1,
        turbine_inlet_temp_c: 1_050.0,
        mass_flow_kg_s: thrust_kn / 1.05,
        sfc_kg_kn_h: 0.090,
    }
}
