#[derive(Debug, Clone, Copy)]
pub struct ThermalExpansion {
    pub formula: &'static str,
    pub alpha_per_k: f64,
    pub temperature_k: f64,
    pub phase: &'static str,
}

pub const TABLE: &[ThermalExpansion] = &[
    ThermalExpansion { formula: "Al",    alpha_per_k: 23.1e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Cu",    alpha_per_k: 16.5e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Fe",    alpha_per_k: 11.8e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Au",    alpha_per_k: 14.2e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Ag",    alpha_per_k: 18.9e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Ni",    alpha_per_k: 13.4e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Ti",    alpha_per_k: 8.6e-6,  temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Mg",    alpha_per_k: 24.8e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Zn",    alpha_per_k: 30.2e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Pb",    alpha_per_k: 28.9e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Sn",    alpha_per_k: 22.0e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "W",     alpha_per_k: 4.5e-6,  temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Mo",    alpha_per_k: 4.8e-6,  temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Pt",    alpha_per_k: 8.8e-6,  temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Si",    alpha_per_k: 2.6e-6,  temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "C",     alpha_per_k: 1.0e-6,  temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "SiO2",  alpha_per_k: 0.55e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "Al2O3", alpha_per_k: 8.1e-6,  temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "MgO",   alpha_per_k: 13.5e-6, temperature_k: 298.15, phase: "solid" },
    ThermalExpansion { formula: "NaCl",  alpha_per_k: 39.6e-6, temperature_k: 298.15, phase: "solid" },

    ThermalExpansion { formula: "H2O",    alpha_per_k: 207.0e-6,  temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "Hg",     alpha_per_k: 181.0e-6,  temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "CH3OH",  alpha_per_k: 1190.0e-6, temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "C2H5OH", alpha_per_k: 1099.0e-6, temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "C3H8O3", alpha_per_k: 504.0e-6,  temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "C6H6",   alpha_per_k: 1240.0e-6, temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "C7H8",   alpha_per_k: 1080.0e-6, temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "C3H6O",  alpha_per_k: 1430.0e-6, temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "CCl4",   alpha_per_k: 1230.0e-6, temperature_k: 298.15, phase: "liquid" },
    ThermalExpansion { formula: "CHCl3",  alpha_per_k: 1273.0e-6, temperature_k: 298.15, phase: "liquid" },
];

pub fn by_formula(formula: &str) -> Option<&'static ThermalExpansion> {
    TABLE.iter().find(|t| t.formula == formula)
}

pub fn by_formula_phase(formula: &str, phase: &str) -> Option<&'static ThermalExpansion> {
    TABLE.iter().find(|t| t.formula == formula && t.phase == phase)
}
