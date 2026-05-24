#[derive(Debug, Clone, Copy)]
pub struct MagneticSusceptibility {
    pub formula: &'static str,
    pub chi_volume: f64,
    pub kind: &'static str,
    pub temperature_k: f64,
}

pub const TABLE: &[MagneticSusceptibility] = &[
    MagneticSusceptibility { formula: "Fe",    chi_volume:  2.0e5,    kind: "ferromagnetic",  temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Co",    chi_volume:  7.0e4,    kind: "ferromagnetic",  temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Ni",    chi_volume:  6.0e2,    kind: "ferromagnetic",  temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Gd",    chi_volume:  4.8e5,    kind: "ferromagnetic",  temperature_k: 200.0  },
    MagneticSusceptibility { formula: "Mn",    chi_volume:  9.5e-4,   kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Cr",    chi_volume:  3.1e-4,   kind: "antiferromagnetic", temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Al",    chi_volume:  2.07e-5,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Pt",    chi_volume:  2.79e-4,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "W",     chi_volume:  7.80e-5,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Ti",    chi_volume:  1.81e-4,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Mg",    chi_volume:  1.20e-5,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Na",    chi_volume:  8.50e-6,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "K",     chi_volume:  5.74e-6,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "U",     chi_volume:  4.10e-4,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "O2",    chi_volume:  1.86e-6,  kind: "paramagnetic",   temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Cu",    chi_volume: -9.63e-6,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Au",    chi_volume: -3.45e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Ag",    chi_volume: -2.38e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Pb",    chi_volume: -1.80e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Bi",    chi_volume: -1.66e-4,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Hg",    chi_volume: -2.84e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Zn",    chi_volume: -1.56e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "C",     chi_volume: -2.10e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Si",    chi_volume: -3.40e-6,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "H2O",   chi_volume: -9.04e-6,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "NaCl",  chi_volume: -1.39e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "SiO2",  chi_volume: -1.40e-5,  kind: "diamagnetic",    temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Fe3O4", chi_volume:  1.0e3,    kind: "ferrimagnetic",  temperature_k: 298.15 },
    MagneticSusceptibility { formula: "Fe2O3", chi_volume:  1.4e-3,   kind: "antiferromagnetic", temperature_k: 298.15 },
    MagneticSusceptibility { formula: "MnO",   chi_volume:  4.4e-3,   kind: "antiferromagnetic", temperature_k: 298.15 },
];

pub fn by_formula(formula: &str) -> Option<&'static MagneticSusceptibility> {
    TABLE.iter().find(|s| s.formula == formula)
}
