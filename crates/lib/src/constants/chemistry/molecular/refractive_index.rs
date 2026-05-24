#[derive(Debug, Clone, Copy)]
pub struct RefractiveIndex {
    pub formula: &'static str,
    pub n_d: f64,
    pub temperature_k: f64,
    pub phase: &'static str,
}

pub const TABLE: &[RefractiveIndex] = &[
    RefractiveIndex { formula: "AIR",    n_d: 1.000277, temperature_k: 288.15, phase: "gas" },
    RefractiveIndex { formula: "He",     n_d: 1.000035, temperature_k: 273.15, phase: "gas" },
    RefractiveIndex { formula: "N2",     n_d: 1.000298, temperature_k: 273.15, phase: "gas" },
    RefractiveIndex { formula: "O2",     n_d: 1.000272, temperature_k: 273.15, phase: "gas" },
    RefractiveIndex { formula: "CO2",    n_d: 1.000449, temperature_k: 273.15, phase: "gas" },

    RefractiveIndex { formula: "H2O",    n_d: 1.3330,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "Hg",     n_d: 1.6200,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "CH3OH",  n_d: 1.3284,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C2H5OH", n_d: 1.3614,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C3H8O3", n_d: 1.4729,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C6H6",   n_d: 1.5011,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C7H8",   n_d: 1.4969,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C8H10",  n_d: 1.4972,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C6H14",  n_d: 1.3749,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C7H16",  n_d: 1.3876,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C8H18",  n_d: 1.3974,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "CHCl3",  n_d: 1.4458,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "CCl4",   n_d: 1.4607,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "CH2Cl2", n_d: 1.4242,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C3H6O",  n_d: 1.3588,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "CS2",    n_d: 1.6276,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "Br2",    n_d: 1.6610,   temperature_k: 293.15, phase: "liquid" },
    RefractiveIndex { formula: "C2H4O2", n_d: 1.3716,   temperature_k: 293.15, phase: "liquid" },

    RefractiveIndex { formula: "SiO2",   n_d: 1.4585,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "Al2O3",  n_d: 1.7700,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "TiO2",   n_d: 2.6140,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "C",      n_d: 2.4170,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "NaCl",   n_d: 1.5442,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "KCl",    n_d: 1.4900,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "CaF2",   n_d: 1.4338,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "MgO",    n_d: 1.7350,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "ZnO",    n_d: 2.0080,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "Si",     n_d: 3.4200,   temperature_k: 293.15, phase: "solid" },
    RefractiveIndex { formula: "Ge",     n_d: 4.0500,   temperature_k: 293.15, phase: "solid" },
];

pub fn by_formula(formula: &str) -> Option<&'static RefractiveIndex> {
    TABLE.iter().find(|r| r.formula == formula)
}

pub fn by_formula_phase(formula: &str, phase: &str) -> Option<&'static RefractiveIndex> {
    TABLE.iter().find(|r| r.formula == formula && r.phase == phase)
}
