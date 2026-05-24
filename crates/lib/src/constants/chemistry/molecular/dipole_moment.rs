#[derive(Debug, Clone, Copy)]
pub struct DipoleMoment {
    pub formula: &'static str,
    pub mu_debye: f64,
    pub phase: &'static str,
}

pub const TABLE: &[DipoleMoment] = &[
    DipoleMoment { formula: "H2",     mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "N2",     mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "O2",     mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "Cl2",    mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "F2",     mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "CO2",    mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "CH4",    mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "C2H4",   mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "C2H2",   mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "C6H6",   mu_debye: 0.000, phase: "liquid" },
    DipoleMoment { formula: "CCl4",   mu_debye: 0.000, phase: "liquid" },
    DipoleMoment { formula: "SF6",    mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "He",     mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "Ar",     mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "H2O",    mu_debye: 1.855, phase: "liquid" },
    DipoleMoment { formula: "NH3",    mu_debye: 1.471, phase: "gas" },
    DipoleMoment { formula: "HF",     mu_debye: 1.826, phase: "gas" },
    DipoleMoment { formula: "HCl",    mu_debye: 1.109, phase: "gas" },
    DipoleMoment { formula: "HBr",    mu_debye: 0.827, phase: "gas" },
    DipoleMoment { formula: "HI",     mu_debye: 0.448, phase: "gas" },
    DipoleMoment { formula: "HCN",    mu_debye: 2.984, phase: "gas" },
    DipoleMoment { formula: "CO",     mu_debye: 0.110, phase: "gas" },
    DipoleMoment { formula: "NO",     mu_debye: 0.159, phase: "gas" },
    DipoleMoment { formula: "NO2",    mu_debye: 0.316, phase: "gas" },
    DipoleMoment { formula: "N2O",    mu_debye: 0.161, phase: "gas" },
    DipoleMoment { formula: "SO2",    mu_debye: 1.633, phase: "gas" },
    DipoleMoment { formula: "SO3",    mu_debye: 0.000, phase: "gas" },
    DipoleMoment { formula: "H2S",    mu_debye: 0.978, phase: "gas" },
    DipoleMoment { formula: "O3",     mu_debye: 0.534, phase: "gas" },
    DipoleMoment { formula: "PH3",    mu_debye: 0.574, phase: "gas" },
    DipoleMoment { formula: "CH3OH",  mu_debye: 1.700, phase: "liquid" },
    DipoleMoment { formula: "C2H5OH", mu_debye: 1.690, phase: "liquid" },
    DipoleMoment { formula: "C3H6O",  mu_debye: 2.880, phase: "liquid" },
    DipoleMoment { formula: "CH2O",   mu_debye: 2.330, phase: "gas" },
    DipoleMoment { formula: "CHCl3",  mu_debye: 1.040, phase: "liquid" },
    DipoleMoment { formula: "CH2Cl2", mu_debye: 1.600, phase: "liquid" },
    DipoleMoment { formula: "C7H8",   mu_debye: 0.375, phase: "liquid" },
    DipoleMoment { formula: "C2H4O2", mu_debye: 1.700, phase: "liquid" },
    DipoleMoment { formula: "C3H8O3", mu_debye: 2.560, phase: "liquid" },
    DipoleMoment { formula: "CH4N2O", mu_debye: 4.560, phase: "solid" },
];

pub fn by_formula(formula: &str) -> Option<&'static DipoleMoment> {
    TABLE.iter().find(|d| d.formula == formula)
}
