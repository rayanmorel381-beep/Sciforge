#[derive(Debug, Clone, Copy)]
pub struct PaschenCoeffs {
    pub formula: &'static str,
    pub a_per_pa_m: f64,
    pub b_v_per_pa_m: f64,
    pub gamma: f64,
}

pub const TABLE: &[PaschenCoeffs] = &[
    PaschenCoeffs { formula: "AIR", a_per_pa_m: 11.25, b_v_per_pa_m: 273.75, gamma: 0.01 },
    PaschenCoeffs { formula: "N2",  a_per_pa_m: 9.00,  b_v_per_pa_m: 257.00, gamma: 0.01 },
    PaschenCoeffs { formula: "O2",  a_per_pa_m: 6.50,  b_v_per_pa_m: 165.00, gamma: 0.02 },
    PaschenCoeffs { formula: "Ar",  a_per_pa_m: 11.50, b_v_per_pa_m: 176.00, gamma: 0.07 },
    PaschenCoeffs { formula: "He",  a_per_pa_m: 2.80,  b_v_per_pa_m:  34.00, gamma: 0.07 },
    PaschenCoeffs { formula: "Ne",  a_per_pa_m: 3.00,  b_v_per_pa_m:  75.00, gamma: 0.07 },
    PaschenCoeffs { formula: "H2",  a_per_pa_m: 3.75,  b_v_per_pa_m: 98.50,  gamma: 0.01 },
    PaschenCoeffs { formula: "CO2", a_per_pa_m: 15.00, b_v_per_pa_m: 350.00, gamma: 0.01 },
    PaschenCoeffs { formula: "SF6", a_per_pa_m: 22.50, b_v_per_pa_m: 880.00, gamma: 0.005 },
];

pub fn by_formula(formula: &str) -> Option<&'static PaschenCoeffs> {
    TABLE.iter().find(|p| p.formula == formula)
}
