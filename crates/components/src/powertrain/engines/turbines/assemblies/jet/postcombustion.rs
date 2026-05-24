use super::turbojet::{Turbojet, TurbojetType};

pub fn fighter(dry_kn: f64, wet_kn: f64, pressure_ratio: f64) -> Turbojet {
    Turbojet {
        jet_type: TurbojetType::WithPostcombustion,
        thrust_kn: dry_kn,
        thrust_afterburner_kn: Some(wet_kn),
        pressure_ratio,
        turbine_inlet_temp_c: 1_350.0,
        mass_flow_kg_s: dry_kn / 1.15,
        sfc_kg_kn_h: 0.19,
    }
}

pub fn interceptor(dry_kn: f64, wet_kn: f64, pressure_ratio: f64) -> Turbojet {
    Turbojet {
        jet_type: TurbojetType::WithPostcombustion,
        thrust_kn: dry_kn,
        thrust_afterburner_kn: Some(wet_kn),
        pressure_ratio,
        turbine_inlet_temp_c: 1_400.0,
        mass_flow_kg_s: dry_kn / 1.1,
        sfc_kg_kn_h: 0.21,
    }
}
