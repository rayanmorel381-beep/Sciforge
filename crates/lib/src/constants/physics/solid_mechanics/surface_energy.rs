#[derive(Debug, Clone, Copy)]
pub struct SurfaceEnergy {
    pub formula: &'static str,
    pub gamma_j_per_m2: f64,
    pub temperature_k: f64,
}

pub const TABLE: &[SurfaceEnergy] = &[
    SurfaceEnergy { formula: "Fe",       gamma_j_per_m2: 2.42, temperature_k: 1823.0 },
    SurfaceEnergy { formula: "Ni",       gamma_j_per_m2: 2.38, temperature_k: 1773.0 },
    SurfaceEnergy { formula: "Cu",       gamma_j_per_m2: 1.83, temperature_k: 1373.0 },
    SurfaceEnergy { formula: "Al",       gamma_j_per_m2: 1.14, temperature_k: 933.0 },
    SurfaceEnergy { formula: "Ti",       gamma_j_per_m2: 2.10, temperature_k: 1973.0 },
    SurfaceEnergy { formula: "Au",       gamma_j_per_m2: 1.50, temperature_k: 1336.0 },
    SurfaceEnergy { formula: "Ag",       gamma_j_per_m2: 1.25, temperature_k: 1235.0 },
    SurfaceEnergy { formula: "Pt",       gamma_j_per_m2: 2.49, temperature_k: 2045.0 },
    SurfaceEnergy { formula: "W",        gamma_j_per_m2: 3.07, temperature_k: 3695.0 },
    SurfaceEnergy { formula: "Mg",       gamma_j_per_m2: 0.78, temperature_k: 923.0 },
    SurfaceEnergy { formula: "diamond",  gamma_j_per_m2: 5.30, temperature_k: 298.15 },
    SurfaceEnergy { formula: "Si",       gamma_j_per_m2: 1.24, temperature_k: 298.15 },
    SurfaceEnergy { formula: "Al2O3",    gamma_j_per_m2: 0.95, temperature_k: 298.15 },
    SurfaceEnergy { formula: "MgO",      gamma_j_per_m2: 1.20, temperature_k: 298.15 },
    SurfaceEnergy { formula: "SiO2",     gamma_j_per_m2: 0.31, temperature_k: 298.15 },
    SurfaceEnergy { formula: "NaCl",     gamma_j_per_m2: 0.30, temperature_k: 298.15 },
    SurfaceEnergy { formula: "PE",       gamma_j_per_m2: 0.033,temperature_k: 293.15 },
    SurfaceEnergy { formula: "PTFE",     gamma_j_per_m2: 0.018,temperature_k: 293.15 },
    SurfaceEnergy { formula: "PMMA",     gamma_j_per_m2: 0.041,temperature_k: 293.15 },
    SurfaceEnergy { formula: "glass_soda",gamma_j_per_m2: 0.30,temperature_k: 298.15 },
    SurfaceEnergy { formula: "H2O",      gamma_j_per_m2: 0.0728,temperature_k: 293.15 },
    SurfaceEnergy { formula: "Hg",       gamma_j_per_m2: 0.4855,temperature_k: 298.15 },
];

pub fn by_formula(formula: &str) -> Option<&'static SurfaceEnergy> {
    TABLE.iter().find(|s| s.formula == formula)
}
