#[derive(Debug, Clone, Copy)]
pub struct Thermoelectric {
    pub formula: &'static str,
    pub seebeck_coefficient_v_per_k: f64,
    pub figure_of_merit_zt: f64,
    pub temperature_k: f64,
}

pub const TABLE: &[Thermoelectric] = &[
    Thermoelectric { formula: "Bi2Te3",   seebeck_coefficient_v_per_k:  2.20e-4, figure_of_merit_zt: 1.00, temperature_k: 300.0 },
    Thermoelectric { formula: "Sb2Te3",   seebeck_coefficient_v_per_k:  1.85e-4, figure_of_merit_zt: 0.80, temperature_k: 300.0 },
    Thermoelectric { formula: "PbTe",     seebeck_coefficient_v_per_k: -1.85e-4, figure_of_merit_zt: 0.80, temperature_k: 700.0 },
    Thermoelectric { formula: "SnSe",     seebeck_coefficient_v_per_k:  5.00e-4, figure_of_merit_zt: 2.60, temperature_k: 923.0 },
    Thermoelectric { formula: "SiGe",     seebeck_coefficient_v_per_k: -3.00e-4, figure_of_merit_zt: 1.00, temperature_k: 1100.0 },
    Thermoelectric { formula: "Mg2Si",    seebeck_coefficient_v_per_k: -2.00e-4, figure_of_merit_zt: 0.70, temperature_k: 800.0 },
    Thermoelectric { formula: "Cu",       seebeck_coefficient_v_per_k:  6.50e-6, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
    Thermoelectric { formula: "Constantan",seebeck_coefficient_v_per_k:-3.50e-5, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
    Thermoelectric { formula: "Chromel",  seebeck_coefficient_v_per_k:  2.20e-5, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
    Thermoelectric { formula: "Alumel",   seebeck_coefficient_v_per_k: -1.80e-5, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
    Thermoelectric { formula: "TypeK",    seebeck_coefficient_v_per_k:  4.10e-5, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
    Thermoelectric { formula: "TypeJ",    seebeck_coefficient_v_per_k:  5.10e-5, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
    Thermoelectric { formula: "TypeT",    seebeck_coefficient_v_per_k:  4.30e-5, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
    Thermoelectric { formula: "TypeS",    seebeck_coefficient_v_per_k:  6.00e-6, figure_of_merit_zt: 0.0,  temperature_k: 300.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static Thermoelectric> {
    TABLE.iter().find(|t| t.formula == formula)
}
