#[derive(Debug, Clone, Copy)]
pub struct OpticalAbsorption {
    pub formula: &'static str,
    pub wavelength_nm: f64,
    pub absorption_coeff_per_m: f64,
    pub extinction_coeff_k: f64,
}

pub const TABLE: &[OpticalAbsorption] = &[
    OpticalAbsorption { formula: "Si",   wavelength_nm: 500.0,  absorption_coeff_per_m: 1.0e6,  extinction_coeff_k: 4.0e-2 },
    OpticalAbsorption { formula: "Si",   wavelength_nm: 1000.0, absorption_coeff_per_m: 6.0e3,  extinction_coeff_k: 4.0e-4 },
    OpticalAbsorption { formula: "Ge",   wavelength_nm: 1550.0, absorption_coeff_per_m: 5.0e5,  extinction_coeff_k: 6.0e-2 },
    OpticalAbsorption { formula: "GaAs", wavelength_nm: 800.0,  absorption_coeff_per_m: 1.0e6,  extinction_coeff_k: 6.0e-2 },
    OpticalAbsorption { formula: "SiO2", wavelength_nm: 633.0,  absorption_coeff_per_m: 1.0e-2, extinction_coeff_k: 1.0e-9 },
    OpticalAbsorption { formula: "SiO2", wavelength_nm: 1550.0, absorption_coeff_per_m: 4.6e-5, extinction_coeff_k: 5.7e-12 },
    OpticalAbsorption { formula: "H2O",  wavelength_nm: 633.0,  absorption_coeff_per_m: 0.36,   extinction_coeff_k: 1.81e-8 },
    OpticalAbsorption { formula: "H2O",  wavelength_nm: 1450.0, absorption_coeff_per_m: 30.0,   extinction_coeff_k: 3.46e-6 },
    OpticalAbsorption { formula: "Al",   wavelength_nm: 550.0,  absorption_coeff_per_m: 1.4e8,  extinction_coeff_k: 6.10 },
    OpticalAbsorption { formula: "Au",   wavelength_nm: 550.0,  absorption_coeff_per_m: 5.0e7,  extinction_coeff_k: 2.21 },
    OpticalAbsorption { formula: "Ag",   wavelength_nm: 550.0,  absorption_coeff_per_m: 8.0e7,  extinction_coeff_k: 3.51 },
    OpticalAbsorption { formula: "Cu",   wavelength_nm: 550.0,  absorption_coeff_per_m: 6.0e7,  extinction_coeff_k: 2.60 },
];

pub fn by_formula_wavelength(formula: &str, wavelength_nm: f64) -> Option<&'static OpticalAbsorption> {
    TABLE.iter()
        .filter(|a| a.formula == formula)
        .min_by(|a, b| {
            (a.wavelength_nm - wavelength_nm).abs()
                .partial_cmp(&(b.wavelength_nm - wavelength_nm).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}
