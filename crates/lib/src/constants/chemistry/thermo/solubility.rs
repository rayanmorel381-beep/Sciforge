#[derive(Debug, Clone, Copy)]
pub struct HenryConstant {
    pub formula: &'static str,
    pub kh_mol_per_kg_pa: f64,
    pub temperature_k: f64,
}

pub const HENRY_TABLE: &[HenryConstant] = &[
    HenryConstant { formula: "O2",   kh_mol_per_kg_pa: 1.3e-8,  temperature_k: 298.15 },
    HenryConstant { formula: "N2",   kh_mol_per_kg_pa: 6.4e-9,  temperature_k: 298.15 },
    HenryConstant { formula: "H2",   kh_mol_per_kg_pa: 7.8e-9,  temperature_k: 298.15 },
    HenryConstant { formula: "CO2",  kh_mol_per_kg_pa: 3.3e-7,  temperature_k: 298.15 },
    HenryConstant { formula: "CO",   kh_mol_per_kg_pa: 9.7e-9,  temperature_k: 298.15 },
    HenryConstant { formula: "CH4",  kh_mol_per_kg_pa: 1.4e-8,  temperature_k: 298.15 },
    HenryConstant { formula: "C2H6", kh_mol_per_kg_pa: 1.9e-8,  temperature_k: 298.15 },
    HenryConstant { formula: "NO",   kh_mol_per_kg_pa: 1.9e-8,  temperature_k: 298.15 },
    HenryConstant { formula: "N2O",  kh_mol_per_kg_pa: 2.4e-7,  temperature_k: 298.15 },
    HenryConstant { formula: "NH3",  kh_mol_per_kg_pa: 5.9e-4,  temperature_k: 298.15 },
    HenryConstant { formula: "H2S",  kh_mol_per_kg_pa: 1.0e-6,  temperature_k: 298.15 },
    HenryConstant { formula: "SO2",  kh_mol_per_kg_pa: 1.2e-5,  temperature_k: 298.15 },
    HenryConstant { formula: "Cl2",  kh_mol_per_kg_pa: 9.5e-7,  temperature_k: 298.15 },
    HenryConstant { formula: "HCl",  kh_mol_per_kg_pa: 1.1e-2,  temperature_k: 298.15 },
    HenryConstant { formula: "He",   kh_mol_per_kg_pa: 3.7e-9,  temperature_k: 298.15 },
    HenryConstant { formula: "Ne",   kh_mol_per_kg_pa: 4.5e-9,  temperature_k: 298.15 },
    HenryConstant { formula: "Ar",   kh_mol_per_kg_pa: 1.4e-8,  temperature_k: 298.15 },
    HenryConstant { formula: "Kr",   kh_mol_per_kg_pa: 2.5e-8,  temperature_k: 298.15 },
    HenryConstant { formula: "Xe",   kh_mol_per_kg_pa: 4.3e-8,  temperature_k: 298.15 },
    HenryConstant { formula: "O3",   kh_mol_per_kg_pa: 1.0e-7,  temperature_k: 298.15 },
];

#[derive(Debug, Clone, Copy)]
pub struct SolubilityProduct {
    pub formula: &'static str,
    pub ksp: f64,
    pub temperature_k: f64,
}

pub const KSP_TABLE: &[SolubilityProduct] = &[
    SolubilityProduct { formula: "AgCl",       ksp: 1.77e-10, temperature_k: 298.15 },
    SolubilityProduct { formula: "AgBr",       ksp: 5.35e-13, temperature_k: 298.15 },
    SolubilityProduct { formula: "AgI",        ksp: 8.52e-17, temperature_k: 298.15 },
    SolubilityProduct { formula: "Ag2S",       ksp: 6.0e-51,  temperature_k: 298.15 },
    SolubilityProduct { formula: "Ag2CO3",     ksp: 8.46e-12, temperature_k: 298.15 },
    SolubilityProduct { formula: "Ag2SO4",     ksp: 1.20e-5,  temperature_k: 298.15 },
    SolubilityProduct { formula: "BaSO4",      ksp: 1.08e-10, temperature_k: 298.15 },
    SolubilityProduct { formula: "BaCO3",      ksp: 2.58e-9,  temperature_k: 298.15 },
    SolubilityProduct { formula: "BaF2",       ksp: 1.84e-7,  temperature_k: 298.15 },
    SolubilityProduct { formula: "CaCO3",      ksp: 3.36e-9,  temperature_k: 298.15 },
    SolubilityProduct { formula: "CaSO4",      ksp: 4.93e-5,  temperature_k: 298.15 },
    SolubilityProduct { formula: "CaF2",       ksp: 3.45e-11, temperature_k: 298.15 },
    SolubilityProduct { formula: "Ca(OH)2",    ksp: 5.02e-6,  temperature_k: 298.15 },
    SolubilityProduct { formula: "Ca3(PO4)2",  ksp: 2.07e-33, temperature_k: 298.15 },
    SolubilityProduct { formula: "CaC2O4",     ksp: 2.32e-9,  temperature_k: 298.15 },
    SolubilityProduct { formula: "CuCl",       ksp: 1.72e-7,  temperature_k: 298.15 },
    SolubilityProduct { formula: "CuS",        ksp: 6.3e-36,  temperature_k: 298.15 },
    SolubilityProduct { formula: "Cu(OH)2",    ksp: 2.20e-20, temperature_k: 298.15 },
    SolubilityProduct { formula: "Fe(OH)2",    ksp: 4.87e-17, temperature_k: 298.15 },
    SolubilityProduct { formula: "Fe(OH)3",    ksp: 2.79e-39, temperature_k: 298.15 },
    SolubilityProduct { formula: "FeS",        ksp: 6.3e-18,  temperature_k: 298.15 },
    SolubilityProduct { formula: "Mg(OH)2",    ksp: 5.61e-12, temperature_k: 298.15 },
    SolubilityProduct { formula: "MgCO3",      ksp: 6.82e-6,  temperature_k: 298.15 },
    SolubilityProduct { formula: "MgF2",       ksp: 5.16e-11, temperature_k: 298.15 },
    SolubilityProduct { formula: "PbCl2",      ksp: 1.70e-5,  temperature_k: 298.15 },
    SolubilityProduct { formula: "PbI2",       ksp: 9.80e-9,  temperature_k: 298.15 },
    SolubilityProduct { formula: "PbSO4",      ksp: 2.53e-8,  temperature_k: 298.15 },
    SolubilityProduct { formula: "PbS",        ksp: 8.0e-28,  temperature_k: 298.15 },
    SolubilityProduct { formula: "ZnS",        ksp: 2.0e-25,  temperature_k: 298.15 },
    SolubilityProduct { formula: "Zn(OH)2",    ksp: 3.0e-17,  temperature_k: 298.15 },
    SolubilityProduct { formula: "Al(OH)3",    ksp: 3.0e-34,  temperature_k: 298.15 },
    SolubilityProduct { formula: "HgS",        ksp: 1.6e-52,  temperature_k: 298.15 },
    SolubilityProduct { formula: "Hg2Cl2",     ksp: 1.43e-18, temperature_k: 298.15 },
    SolubilityProduct { formula: "SrSO4",      ksp: 3.44e-7,  temperature_k: 298.15 },
    SolubilityProduct { formula: "SrCO3",      ksp: 5.60e-10, temperature_k: 298.15 },
    SolubilityProduct { formula: "NiS",        ksp: 4.0e-20,  temperature_k: 298.15 },
];

pub fn henry_by_formula(formula: &str) -> Option<&'static HenryConstant> {
    HENRY_TABLE.iter().find(|h| h.formula == formula)
}

pub fn ksp_by_formula(formula: &str) -> Option<&'static SolubilityProduct> {
    KSP_TABLE.iter().find(|s| s.formula == formula)
}
