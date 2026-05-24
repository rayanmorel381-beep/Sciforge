#[derive(Debug, Clone, Copy)]
pub struct DielectricConstant {
    pub formula: &'static str,
    pub epsilon_r: f64,
    pub temperature_k: f64,
    pub phase: &'static str,
}

pub const TABLE: &[DielectricConstant] = &[
    DielectricConstant { formula: "AIR",    epsilon_r: 1.00059,  temperature_k: 293.15, phase: "gas" },
    DielectricConstant { formula: "H2",     epsilon_r: 1.000270, temperature_k: 273.15, phase: "gas" },
    DielectricConstant { formula: "N2",     epsilon_r: 1.000548, temperature_k: 273.15, phase: "gas" },
    DielectricConstant { formula: "O2",     epsilon_r: 1.000523, temperature_k: 293.15, phase: "gas" },
    DielectricConstant { formula: "CO2",    epsilon_r: 1.000922, temperature_k: 273.15, phase: "gas" },
    DielectricConstant { formula: "H2O",    epsilon_r: 1.00785,  temperature_k: 373.15, phase: "gas" },

    DielectricConstant { formula: "H2O",    epsilon_r: 78.36,    temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "CH3OH",  epsilon_r: 32.66,    temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C2H5OH", epsilon_r: 24.55,    temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C3H8O3", epsilon_r: 42.50,    temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "NH3",    epsilon_r: 16.90,    temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C3H6O",  epsilon_r: 20.70,    temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "CH2O",   epsilon_r: 8.50,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C2H4O2", epsilon_r: 6.20,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C6H6",   epsilon_r: 2.27,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C7H8",   epsilon_r: 2.38,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "CCl4",   epsilon_r: 2.24,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "CHCl3",  epsilon_r: 4.81,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "CH2Cl2", epsilon_r: 8.93,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C6H14",  epsilon_r: 1.89,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "C8H18",  epsilon_r: 1.95,     temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "Hg",     epsilon_r: 1.00074,  temperature_k: 298.15, phase: "liquid" },
    DielectricConstant { formula: "CS2",    epsilon_r: 2.64,     temperature_k: 298.15, phase: "liquid" },

    DielectricConstant { formula: "SiO2",   epsilon_r: 3.90,     temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "Al2O3",  epsilon_r: 9.80,     temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "TiO2",   epsilon_r: 86.00,    temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "MgO",    epsilon_r: 9.65,     temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "C",      epsilon_r: 5.70,     temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "Si",     epsilon_r: 11.68,    temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "Ge",     epsilon_r: 16.20,    temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "NaCl",   epsilon_r: 5.90,     temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "KCl",    epsilon_r: 4.68,     temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "CaF2",   epsilon_r: 6.81,     temperature_k: 298.15, phase: "solid" },
    DielectricConstant { formula: "BaTiO3", epsilon_r: 1700.0,   temperature_k: 298.15, phase: "solid" },
];

pub fn by_formula(formula: &str) -> Option<&'static DielectricConstant> {
    TABLE.iter().find(|d| d.formula == formula)
}

pub fn by_formula_phase(formula: &str, phase: &str) -> Option<&'static DielectricConstant> {
    TABLE.iter().find(|d| d.formula == formula && d.phase == phase)
}
