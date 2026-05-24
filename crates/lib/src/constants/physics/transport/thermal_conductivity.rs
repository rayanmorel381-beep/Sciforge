#[derive(Debug, Clone, Copy)]
pub struct ThermalConductivity {
    pub formula: &'static str,
    pub k_w_per_mk: f64,
    pub temperature_k: f64,
    pub phase: &'static str,
}

pub const TABLE: &[ThermalConductivity] = &[
    ThermalConductivity { formula: "AIR",  k_w_per_mk: 0.02624, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "N2",   k_w_per_mk: 0.02583, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "O2",   k_w_per_mk: 0.02658, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "H2",   k_w_per_mk: 0.18620, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "He",   k_w_per_mk: 0.15400, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "Ne",   k_w_per_mk: 0.04930, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "Ar",   k_w_per_mk: 0.01772, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "Kr",   k_w_per_mk: 0.00943, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "Xe",   k_w_per_mk: 0.00565, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "CO2",  k_w_per_mk: 0.01663, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "CO",   k_w_per_mk: 0.02500, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "H2O",  k_w_per_mk: 0.01963, temperature_k: 373.15, phase: "gas" },
    ThermalConductivity { formula: "NH3",  k_w_per_mk: 0.02468, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "CH4",  k_w_per_mk: 0.03430, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "C2H6", k_w_per_mk: 0.02100, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "SO2",  k_w_per_mk: 0.00961, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "Cl2",  k_w_per_mk: 0.00891, temperature_k: 300.0,  phase: "gas" },
    ThermalConductivity { formula: "SF6",  k_w_per_mk: 0.01300, temperature_k: 300.0,  phase: "gas" },

    ThermalConductivity { formula: "H2O",    k_w_per_mk: 0.6065,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "Hg",     k_w_per_mk: 8.30,    temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "CH3OH",  k_w_per_mk: 0.2020,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "C2H5OH", k_w_per_mk: 0.1690,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "C3H8O3", k_w_per_mk: 0.2860,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "C6H6",   k_w_per_mk: 0.1410,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "C7H8",   k_w_per_mk: 0.1310,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "CHCl3",  k_w_per_mk: 0.1170,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "CCl4",   k_w_per_mk: 0.0990,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "C3H6O",  k_w_per_mk: 0.1610,  temperature_k: 298.15, phase: "liquid" },
    ThermalConductivity { formula: "NH3",    k_w_per_mk: 0.4970,  temperature_k: 239.81, phase: "liquid" },

    ThermalConductivity { formula: "Ag",    k_w_per_mk: 429.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Cu",    k_w_per_mk: 401.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Au",    k_w_per_mk: 318.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Al",    k_w_per_mk: 237.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "W",     k_w_per_mk: 173.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Mg",    k_w_per_mk: 156.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Si",    k_w_per_mk: 149.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Mo",    k_w_per_mk: 138.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Zn",    k_w_per_mk: 116.0,  temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Ni",    k_w_per_mk: 90.9,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Fe",    k_w_per_mk: 80.4,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Pt",    k_w_per_mk: 71.6,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Sn",    k_w_per_mk: 66.8,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Pb",    k_w_per_mk: 35.3,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Ti",    k_w_per_mk: 21.9,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "C",     k_w_per_mk: 1950.0, temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "SiO2",  k_w_per_mk: 1.40,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Al2O3", k_w_per_mk: 30.0,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "Fe2O3", k_w_per_mk: 12.6,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "MgO",   k_w_per_mk: 30.0,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "TiO2",  k_w_per_mk: 11.7,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "NaCl",  k_w_per_mk: 6.50,   temperature_k: 298.15, phase: "solid" },
    ThermalConductivity { formula: "CaCO3", k_w_per_mk: 3.59,   temperature_k: 298.15, phase: "solid" },
];

pub fn by_formula(formula: &str) -> Option<&'static ThermalConductivity> {
    TABLE.iter().find(|t| t.formula == formula)
}

pub fn by_formula_phase(formula: &str, phase: &str) -> Option<&'static ThermalConductivity> {
    TABLE.iter().find(|t| t.formula == formula && t.phase == phase)
}
