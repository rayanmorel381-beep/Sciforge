#[derive(Debug, Clone, Copy)]
pub struct DebyeTemperature {
    pub formula: &'static str,
    pub theta_d_k: f64,
}

pub const TABLE: &[DebyeTemperature] = &[
    DebyeTemperature { formula: "Li", theta_d_k:  344.0 },
    DebyeTemperature { formula: "Be", theta_d_k: 1440.0 },
    DebyeTemperature { formula: "Na", theta_d_k:  158.0 },
    DebyeTemperature { formula: "Mg", theta_d_k:  400.0 },
    DebyeTemperature { formula: "Al", theta_d_k:  428.0 },
    DebyeTemperature { formula: "Si", theta_d_k:  645.0 },
    DebyeTemperature { formula: "K",  theta_d_k:   91.0 },
    DebyeTemperature { formula: "Ca", theta_d_k:  230.0 },
    DebyeTemperature { formula: "Ti", theta_d_k:  420.0 },
    DebyeTemperature { formula: "V",  theta_d_k:  380.0 },
    DebyeTemperature { formula: "Cr", theta_d_k:  630.0 },
    DebyeTemperature { formula: "Mn", theta_d_k:  410.0 },
    DebyeTemperature { formula: "Fe", theta_d_k:  470.0 },
    DebyeTemperature { formula: "Co", theta_d_k:  445.0 },
    DebyeTemperature { formula: "Ni", theta_d_k:  450.0 },
    DebyeTemperature { formula: "Cu", theta_d_k:  343.0 },
    DebyeTemperature { formula: "Zn", theta_d_k:  327.0 },
    DebyeTemperature { formula: "Ge", theta_d_k:  374.0 },
    DebyeTemperature { formula: "Mo", theta_d_k:  450.0 },
    DebyeTemperature { formula: "Ag", theta_d_k:  225.0 },
    DebyeTemperature { formula: "Cd", theta_d_k:  209.0 },
    DebyeTemperature { formula: "In", theta_d_k:  108.0 },
    DebyeTemperature { formula: "Sn", theta_d_k:  200.0 },
    DebyeTemperature { formula: "W",  theta_d_k:  400.0 },
    DebyeTemperature { formula: "Pt", theta_d_k:  240.0 },
    DebyeTemperature { formula: "Au", theta_d_k:  165.0 },
    DebyeTemperature { formula: "Hg", theta_d_k:   72.0 },
    DebyeTemperature { formula: "Pb", theta_d_k:  105.0 },
    DebyeTemperature { formula: "C",  theta_d_k: 2230.0 },
    DebyeTemperature { formula: "He", theta_d_k:   30.0 },
    DebyeTemperature { formula: "Ne", theta_d_k:   75.0 },
    DebyeTemperature { formula: "Ar", theta_d_k:   92.0 },
    DebyeTemperature { formula: "Kr", theta_d_k:   72.0 },
    DebyeTemperature { formula: "Xe", theta_d_k:   64.0 },
    DebyeTemperature { formula: "NaCl",theta_d_k: 321.0 },
    DebyeTemperature { formula: "KCl", theta_d_k: 235.0 },
    DebyeTemperature { formula: "MgO", theta_d_k: 946.0 },
    DebyeTemperature { formula: "Al2O3",theta_d_k:1047.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static DebyeTemperature> {
    TABLE.iter().find(|d| d.formula == formula)
}
