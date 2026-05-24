#[derive(Debug, Clone, Copy)]
pub struct SurfaceTension {
    pub formula: &'static str,
    pub sigma_n_per_m: f64,
    pub temperature_k: f64,
}

pub const TABLE: &[SurfaceTension] = &[
    SurfaceTension { formula: "H2O",    sigma_n_per_m: 0.07197, temperature_k: 298.15 },
    SurfaceTension { formula: "Hg",     sigma_n_per_m: 0.486,   temperature_k: 298.15 },
    SurfaceTension { formula: "CH3OH",  sigma_n_per_m: 0.02240, temperature_k: 298.15 },
    SurfaceTension { formula: "C2H5OH", sigma_n_per_m: 0.02195, temperature_k: 298.15 },
    SurfaceTension { formula: "C3H8O3", sigma_n_per_m: 0.0633,  temperature_k: 298.15 },
    SurfaceTension { formula: "C6H6",   sigma_n_per_m: 0.02822, temperature_k: 298.15 },
    SurfaceTension { formula: "C7H8",   sigma_n_per_m: 0.02793, temperature_k: 298.15 },
    SurfaceTension { formula: "C8H10",  sigma_n_per_m: 0.02817, temperature_k: 298.15 },
    SurfaceTension { formula: "CHCl3",  sigma_n_per_m: 0.02614, temperature_k: 298.15 },
    SurfaceTension { formula: "CH2Cl2", sigma_n_per_m: 0.02780, temperature_k: 298.15 },
    SurfaceTension { formula: "CCl4",   sigma_n_per_m: 0.02605, temperature_k: 298.15 },
    SurfaceTension { formula: "C3H6O",  sigma_n_per_m: 0.02266, temperature_k: 298.15 },
    SurfaceTension { formula: "C6H14",  sigma_n_per_m: 0.01789, temperature_k: 298.15 },
    SurfaceTension { formula: "C7H16",  sigma_n_per_m: 0.01996, temperature_k: 298.15 },
    SurfaceTension { formula: "C8H18",  sigma_n_per_m: 0.02134, temperature_k: 298.15 },
    SurfaceTension { formula: "NH3",    sigma_n_per_m: 0.02338, temperature_k: 239.81 },
    SurfaceTension { formula: "N2",     sigma_n_per_m: 0.00890, temperature_k: 77.0 },
    SurfaceTension { formula: "O2",     sigma_n_per_m: 0.01320, temperature_k: 90.0 },
    SurfaceTension { formula: "Ar",     sigma_n_per_m: 0.01290, temperature_k: 87.30 },
    SurfaceTension { formula: "He",     sigma_n_per_m: 0.000353,temperature_k: 4.22 },
    SurfaceTension { formula: "H2",     sigma_n_per_m: 0.00193, temperature_k: 20.27 },
    SurfaceTension { formula: "Ne",     sigma_n_per_m: 0.00549, temperature_k: 27.07 },
    SurfaceTension { formula: "Kr",     sigma_n_per_m: 0.01580, temperature_k: 119.93 },
    SurfaceTension { formula: "Xe",     sigma_n_per_m: 0.01880, temperature_k: 165.03 },
    SurfaceTension { formula: "CO2",    sigma_n_per_m: 0.00120, temperature_k: 298.15 },
    SurfaceTension { formula: "CH4",    sigma_n_per_m: 0.01340, temperature_k: 111.65 },
    SurfaceTension { formula: "C2H6",   sigma_n_per_m: 0.01620, temperature_k: 184.55 },
    SurfaceTension { formula: "C3H8",   sigma_n_per_m: 0.01880, temperature_k: 231.04 },
    SurfaceTension { formula: "C4H10",  sigma_n_per_m: 0.01200, temperature_k: 298.15 },
    SurfaceTension { formula: "C5H12",  sigma_n_per_m: 0.01580, temperature_k: 298.15 },
    SurfaceTension { formula: "C2H4O2", sigma_n_per_m: 0.02750, temperature_k: 298.15 },
    SurfaceTension { formula: "Br2",    sigma_n_per_m: 0.04050, temperature_k: 298.15 },
    SurfaceTension { formula: "CS2",    sigma_n_per_m: 0.03155, temperature_k: 298.15 },
    SurfaceTension { formula: "SO2",    sigma_n_per_m: 0.02500, temperature_k: 263.13 },
    SurfaceTension { formula: "Cl2",    sigma_n_per_m: 0.01840, temperature_k: 239.11 },
];

pub fn by_formula(formula: &str) -> Option<&'static SurfaceTension> {
    TABLE.iter().find(|s| s.formula == formula)
}
