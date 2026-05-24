#[derive(Debug, Clone, Copy)]
pub struct BinaryDiffusivity {
    pub solute: &'static str,
    pub solvent: &'static str,
    pub d_m2_per_s: f64,
    pub temperature_k: f64,
    pub phase: &'static str,
}

pub const TABLE: &[BinaryDiffusivity] = &[
    BinaryDiffusivity { solute: "H2",   solvent: "AIR",  d_m2_per_s: 6.11e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "He",   solvent: "AIR",  d_m2_per_s: 6.24e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "O2",   solvent: "AIR",  d_m2_per_s: 1.78e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "N2",   solvent: "AIR",  d_m2_per_s: 1.78e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "CO2",  solvent: "AIR",  d_m2_per_s: 1.38e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "CO",   solvent: "AIR",  d_m2_per_s: 1.85e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "H2O",  solvent: "AIR",  d_m2_per_s: 2.20e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "NH3",  solvent: "AIR",  d_m2_per_s: 1.98e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "SO2",  solvent: "AIR",  d_m2_per_s: 1.22e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "CH4",  solvent: "AIR",  d_m2_per_s: 1.96e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "C2H6", solvent: "AIR",  d_m2_per_s: 1.08e-5,  temperature_k: 273.15, phase: "gas" },
    BinaryDiffusivity { solute: "C6H6", solvent: "AIR",  d_m2_per_s: 8.80e-6,  temperature_k: 298.15, phase: "gas" },
    BinaryDiffusivity { solute: "C2H5OH",solvent: "AIR", d_m2_per_s: 1.19e-5,  temperature_k: 298.15, phase: "gas" },
    BinaryDiffusivity { solute: "Cl2",  solvent: "AIR",  d_m2_per_s: 1.24e-5,  temperature_k: 273.15, phase: "gas" },

    BinaryDiffusivity { solute: "O2",   solvent: "H2O",  d_m2_per_s: 2.10e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "N2",   solvent: "H2O",  d_m2_per_s: 1.88e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "H2",   solvent: "H2O",  d_m2_per_s: 4.50e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "CO2",  solvent: "H2O",  d_m2_per_s: 1.92e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "CH4",  solvent: "H2O",  d_m2_per_s: 1.49e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "NH3",  solvent: "H2O",  d_m2_per_s: 1.64e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "Cl2",  solvent: "H2O",  d_m2_per_s: 1.25e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "NaCl", solvent: "H2O",  d_m2_per_s: 1.61e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "KCl",  solvent: "H2O",  d_m2_per_s: 1.99e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "C6H12O6",solvent:"H2O", d_m2_per_s: 6.73e-10, temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "CH4N2O",solvent: "H2O", d_m2_per_s: 1.38e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "C2H5OH",solvent: "H2O", d_m2_per_s: 1.24e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "CH3OH",solvent: "H2O",  d_m2_per_s: 1.58e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "C3H6O",solvent: "H2O",  d_m2_per_s: 1.28e-9,  temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "C6H6", solvent: "C2H5OH",d_m2_per_s: 1.81e-9, temperature_k: 298.15, phase: "liquid" },
    BinaryDiffusivity { solute: "I2",   solvent: "C6H6", d_m2_per_s: 2.13e-9,  temperature_k: 298.15, phase: "liquid" },
];

pub fn by_pair(solute: &str, solvent: &str) -> Option<&'static BinaryDiffusivity> {
    TABLE.iter().find(|d| d.solute == solute && d.solvent == solvent)
}
