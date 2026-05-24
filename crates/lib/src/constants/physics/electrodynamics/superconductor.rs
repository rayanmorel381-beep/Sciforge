#[derive(Debug, Clone, Copy)]
pub struct Superconductor {
    pub formula: &'static str,
    pub critical_temperature_k: f64,
    pub critical_field_t: f64,
    pub critical_current_density_a_m2: f64,
    pub kind: &'static str,
}

pub const TABLE: &[Superconductor] = &[
    Superconductor { formula: "Al",    critical_temperature_k: 1.20,  critical_field_t: 0.010,  critical_current_density_a_m2: 1.0e9,  kind: "I"  },
    Superconductor { formula: "Pb",    critical_temperature_k: 7.20,  critical_field_t: 0.080,  critical_current_density_a_m2: 1.0e9,  kind: "I"  },
    Superconductor { formula: "Hg",    critical_temperature_k: 4.15,  critical_field_t: 0.041,  critical_current_density_a_m2: 1.0e9,  kind: "I"  },
    Superconductor { formula: "Sn",    critical_temperature_k: 3.72,  critical_field_t: 0.031,  critical_current_density_a_m2: 1.0e9,  kind: "I"  },
    Superconductor { formula: "Nb",    critical_temperature_k: 9.25,  critical_field_t: 0.206,  critical_current_density_a_m2: 1.0e10, kind: "II" },
    Superconductor { formula: "NbTi",  critical_temperature_k: 9.50,  critical_field_t: 14.5,   critical_current_density_a_m2: 3.0e9,  kind: "II" },
    Superconductor { formula: "Nb3Sn", critical_temperature_k: 18.30, critical_field_t: 25.0,   critical_current_density_a_m2: 3.0e9,  kind: "II" },
    Superconductor { formula: "Nb3Ge", critical_temperature_k: 23.20, critical_field_t: 38.0,   critical_current_density_a_m2: 1.0e9,  kind: "II" },
    Superconductor { formula: "MgB2",  critical_temperature_k: 39.00, critical_field_t: 74.0,   critical_current_density_a_m2: 1.0e10, kind: "II" },
    Superconductor { formula: "YBCO",  critical_temperature_k: 92.00, critical_field_t: 120.0,  critical_current_density_a_m2: 3.0e10, kind: "II" },
    Superconductor { formula: "BSCCO", critical_temperature_k: 110.0, critical_field_t: 200.0,  critical_current_density_a_m2: 1.0e10, kind: "II" },
    Superconductor { formula: "TBCCO", critical_temperature_k: 138.0, critical_field_t: 200.0,  critical_current_density_a_m2: 1.0e10, kind: "II" },
    Superconductor { formula: "H3S",   critical_temperature_k: 203.0, critical_field_t: 88.0,   critical_current_density_a_m2: 0.0,    kind: "II" },
];

pub fn by_formula(formula: &str) -> Option<&'static Superconductor> {
    TABLE.iter().find(|s| s.formula == formula)
}
