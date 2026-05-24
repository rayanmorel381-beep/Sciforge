#[derive(Debug, Clone, Copy)]
pub struct ParisLaw {
    pub formula: &'static str,
    pub c: f64,
    pub m: f64,
    pub delta_k_threshold_pa_sqrt_m: f64,
    pub k_ic_pa_sqrt_m: f64,
    pub r_ratio: f64,
}

pub const TABLE: &[ParisLaw] = &[
    ParisLaw { formula: "AISI_1020",   c: 6.9e-12, m: 3.0, delta_k_threshold_pa_sqrt_m: 6.5e6,  k_ic_pa_sqrt_m: 5.4e7,  r_ratio: 0.1 },
    ParisLaw { formula: "AISI_1045",   c: 4.0e-12, m: 3.0, delta_k_threshold_pa_sqrt_m: 7.0e6,  k_ic_pa_sqrt_m: 5.0e7,  r_ratio: 0.1 },
    ParisLaw { formula: "AISI_4340",   c: 5.0e-12, m: 3.0, delta_k_threshold_pa_sqrt_m: 8.0e6,  k_ic_pa_sqrt_m: 7.5e7,  r_ratio: 0.1 },
    ParisLaw { formula: "AISI_304",    c: 5.6e-12, m: 3.25,delta_k_threshold_pa_sqrt_m: 6.0e6,  k_ic_pa_sqrt_m: 1.50e8, r_ratio: 0.1 },
    ParisLaw { formula: "AISI_316",    c: 3.0e-12, m: 3.30,delta_k_threshold_pa_sqrt_m: 6.5e6,  k_ic_pa_sqrt_m: 1.50e8, r_ratio: 0.1 },
    ParisLaw { formula: "Al_6061_T6",  c: 1.6e-11, m: 3.32,delta_k_threshold_pa_sqrt_m: 3.0e6,  k_ic_pa_sqrt_m: 2.9e7,  r_ratio: 0.1 },
    ParisLaw { formula: "Al_7075_T6",  c: 1.0e-11, m: 3.5, delta_k_threshold_pa_sqrt_m: 2.7e6,  k_ic_pa_sqrt_m: 2.5e7,  r_ratio: 0.1 },
    ParisLaw { formula: "Al_2024_T3",  c: 1.6e-11, m: 3.59,delta_k_threshold_pa_sqrt_m: 2.9e6,  k_ic_pa_sqrt_m: 3.7e7,  r_ratio: 0.1 },
    ParisLaw { formula: "Ti_6Al_4V",   c: 6.9e-12, m: 3.5, delta_k_threshold_pa_sqrt_m: 5.0e6,  k_ic_pa_sqrt_m: 7.5e7,  r_ratio: 0.1 },
    ParisLaw { formula: "Inconel_718", c: 4.0e-12, m: 3.0, delta_k_threshold_pa_sqrt_m: 8.0e6,  k_ic_pa_sqrt_m: 1.10e8, r_ratio: 0.1 },
    ParisLaw { formula: "Cu",          c: 5.0e-12, m: 3.5, delta_k_threshold_pa_sqrt_m: 4.0e6,  k_ic_pa_sqrt_m: 4.5e7,  r_ratio: 0.1 },
];

pub fn by_formula(formula: &str) -> Option<&'static ParisLaw> {
    TABLE.iter().find(|p| p.formula == formula)
}
