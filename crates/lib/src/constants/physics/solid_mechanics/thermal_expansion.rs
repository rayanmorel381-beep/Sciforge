#[derive(Debug, Clone, Copy)]
pub struct ThermalExpansion {
    pub formula: &'static str,
    pub alpha_per_k: f64,
    pub alpha_poly_coeffs: [f64; 3],
    pub valid_range_k: (f64, f64),
}

pub const TABLE: &[ThermalExpansion] = &[
    ThermalExpansion { formula: "Fe",          alpha_per_k: 1.18e-5, alpha_poly_coeffs: [1.18e-5, 7.0e-9,  0.0], valid_range_k: (293.0, 1000.0) },
    ThermalExpansion { formula: "AISI_1020",   alpha_per_k: 1.17e-5, alpha_poly_coeffs: [1.17e-5, 6.5e-9,  0.0], valid_range_k: (293.0, 1000.0) },
    ThermalExpansion { formula: "AISI_304",    alpha_per_k: 1.73e-5, alpha_poly_coeffs: [1.73e-5, 8.0e-9,  0.0], valid_range_k: (293.0, 1100.0) },
    ThermalExpansion { formula: "AISI_316",    alpha_per_k: 1.60e-5, alpha_poly_coeffs: [1.60e-5, 7.5e-9,  0.0], valid_range_k: (293.0, 1100.0) },
    ThermalExpansion { formula: "Al",          alpha_per_k: 2.31e-5, alpha_poly_coeffs: [2.31e-5, 1.5e-8,  0.0], valid_range_k: (293.0, 800.0) },
    ThermalExpansion { formula: "Al_6061_T6",  alpha_per_k: 2.36e-5, alpha_poly_coeffs: [2.36e-5, 1.5e-8,  0.0], valid_range_k: (293.0, 800.0) },
    ThermalExpansion { formula: "Cu",          alpha_per_k: 1.65e-5, alpha_poly_coeffs: [1.65e-5, 1.0e-8,  0.0], valid_range_k: (293.0, 1300.0) },
    ThermalExpansion { formula: "Ti",          alpha_per_k: 8.60e-6, alpha_poly_coeffs: [8.60e-6, 4.0e-9,  0.0], valid_range_k: (293.0, 1100.0) },
    ThermalExpansion { formula: "Ti_6Al_4V",   alpha_per_k: 8.60e-6, alpha_poly_coeffs: [8.60e-6, 4.0e-9,  0.0], valid_range_k: (293.0, 1100.0) },
    ThermalExpansion { formula: "Ni",          alpha_per_k: 1.30e-5, alpha_poly_coeffs: [1.30e-5, 5.5e-9,  0.0], valid_range_k: (293.0, 1500.0) },
    ThermalExpansion { formula: "Inconel_718", alpha_per_k: 1.30e-5, alpha_poly_coeffs: [1.30e-5, 5.0e-9,  0.0], valid_range_k: (293.0, 1300.0) },
    ThermalExpansion { formula: "W",           alpha_per_k: 4.50e-6, alpha_poly_coeffs: [4.50e-6, 1.5e-9,  0.0], valid_range_k: (293.0, 2500.0) },
    ThermalExpansion { formula: "Mg",          alpha_per_k: 2.48e-5, alpha_poly_coeffs: [2.48e-5, 1.4e-8,  0.0], valid_range_k: (293.0, 800.0) },
    ThermalExpansion { formula: "Pb",          alpha_per_k: 2.89e-5, alpha_poly_coeffs: [2.89e-5, 2.0e-8,  0.0], valid_range_k: (293.0, 600.0) },
    ThermalExpansion { formula: "Au",          alpha_per_k: 1.42e-5, alpha_poly_coeffs: [1.42e-5, 8.0e-9,  0.0], valid_range_k: (293.0, 1300.0) },
    ThermalExpansion { formula: "Si",          alpha_per_k: 2.60e-6, alpha_poly_coeffs: [2.60e-6, 2.0e-9,  0.0], valid_range_k: (293.0, 1500.0) },
    ThermalExpansion { formula: "SiO2",        alpha_per_k: 5.50e-7, alpha_poly_coeffs: [5.50e-7, 3.0e-10, 0.0], valid_range_k: (293.0, 1500.0) },
    ThermalExpansion { formula: "Al2O3",       alpha_per_k: 8.10e-6, alpha_poly_coeffs: [8.10e-6, 3.0e-9,  0.0], valid_range_k: (293.0, 1800.0) },
    ThermalExpansion { formula: "SiC",         alpha_per_k: 4.00e-6, alpha_poly_coeffs: [4.00e-6, 2.0e-9,  0.0], valid_range_k: (293.0, 1800.0) },
    ThermalExpansion { formula: "Si3N4",       alpha_per_k: 3.30e-6, alpha_poly_coeffs: [3.30e-6, 1.5e-9,  0.0], valid_range_k: (293.0, 1700.0) },
    ThermalExpansion { formula: "diamond",     alpha_per_k: 1.10e-6, alpha_poly_coeffs: [1.10e-6, 1.0e-9,  0.0], valid_range_k: (293.0, 1500.0) },
    ThermalExpansion { formula: "concrete",    alpha_per_k: 1.00e-5, alpha_poly_coeffs: [1.00e-5, 0.0,     0.0], valid_range_k: (273.0, 600.0) },
    ThermalExpansion { formula: "PE",          alpha_per_k: 2.00e-4, alpha_poly_coeffs: [2.00e-4, 0.0,     0.0], valid_range_k: (273.0, 380.0) },
    ThermalExpansion { formula: "PMMA",        alpha_per_k: 7.00e-5, alpha_poly_coeffs: [7.00e-5, 0.0,     0.0], valid_range_k: (273.0, 380.0) },
    ThermalExpansion { formula: "PTFE",        alpha_per_k: 1.35e-4, alpha_poly_coeffs: [1.35e-4, 0.0,     0.0], valid_range_k: (273.0, 500.0) },
    ThermalExpansion { formula: "glass_soda",  alpha_per_k: 9.00e-6, alpha_poly_coeffs: [9.00e-6, 4.0e-9,  0.0], valid_range_k: (293.0, 800.0) },
    ThermalExpansion { formula: "wood_pine",   alpha_per_k: 5.00e-6, alpha_poly_coeffs: [5.00e-6, 0.0,     0.0], valid_range_k: (273.0, 350.0) },
];

pub fn by_formula(formula: &str) -> Option<&'static ThermalExpansion> {
    TABLE.iter().find(|t| t.formula == formula)
}

pub fn alpha_at_temperature(entry: &ThermalExpansion, t_k: f64) -> f64 {
    let dt = t_k - 293.15;
    entry.alpha_poly_coeffs[0] + entry.alpha_poly_coeffs[1] * dt + entry.alpha_poly_coeffs[2] * dt * dt
}
