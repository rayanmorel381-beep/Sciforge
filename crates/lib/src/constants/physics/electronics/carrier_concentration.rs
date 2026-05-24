#[derive(Debug, Clone, Copy)]
pub struct CarrierData {
    pub formula: &'static str,
    pub ni_300k_per_m3: f64,
    pub electron_mobility_m2_per_vs: f64,
    pub hole_mobility_m2_per_vs: f64,
    pub effective_mass_electron: f64,
    pub effective_mass_hole: f64,
}

pub const TABLE: &[CarrierData] = &[
    CarrierData { formula: "Si",   ni_300k_per_m3: 1.0e16, electron_mobility_m2_per_vs: 0.135, hole_mobility_m2_per_vs: 0.048, effective_mass_electron: 1.08, effective_mass_hole: 0.81 },
    CarrierData { formula: "Ge",   ni_300k_per_m3: 2.4e19, electron_mobility_m2_per_vs: 0.390, hole_mobility_m2_per_vs: 0.190, effective_mass_electron: 0.55, effective_mass_hole: 0.37 },
    CarrierData { formula: "GaAs", ni_300k_per_m3: 2.1e12, electron_mobility_m2_per_vs: 0.850, hole_mobility_m2_per_vs: 0.040, effective_mass_electron: 0.067, effective_mass_hole: 0.45 },
    CarrierData { formula: "GaN",  ni_300k_per_m3: 1.0e6,  electron_mobility_m2_per_vs: 0.100, hole_mobility_m2_per_vs: 0.0035,effective_mass_electron: 0.20, effective_mass_hole: 0.80 },
    CarrierData { formula: "InP",  ni_300k_per_m3: 1.3e13, electron_mobility_m2_per_vs: 0.540, hole_mobility_m2_per_vs: 0.020, effective_mass_electron: 0.077, effective_mass_hole: 0.64 },
    CarrierData { formula: "InAs", ni_300k_per_m3: 1.0e21, electron_mobility_m2_per_vs: 4.000, hole_mobility_m2_per_vs: 0.050, effective_mass_electron: 0.023, effective_mass_hole: 0.41 },
    CarrierData { formula: "SiC",  ni_300k_per_m3: 1.0e1,  electron_mobility_m2_per_vs: 0.090, hole_mobility_m2_per_vs: 0.012, effective_mass_electron: 0.39, effective_mass_hole: 1.00 },
    CarrierData { formula: "C",    ni_300k_per_m3: 1.0e-13,electron_mobility_m2_per_vs: 0.220, hole_mobility_m2_per_vs: 0.180, effective_mass_electron: 0.20, effective_mass_hole: 0.25 },
];

pub fn by_formula(formula: &str) -> Option<&'static CarrierData> {
    TABLE.iter().find(|c| c.formula == formula)
}
