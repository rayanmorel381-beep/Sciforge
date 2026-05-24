#[derive(Debug, Clone, Copy)]
pub struct PlasticityHardening {
    pub formula: &'static str,
    pub hollomon_k_pa: f64,
    pub hollomon_n: f64,
    pub jc_a_pa: f64,
    pub jc_b_pa: f64,
    pub jc_n: f64,
    pub jc_c: f64,
    pub jc_m: f64,
    pub jc_t_melt_k: f64,
    pub jc_eps_dot_ref: f64,
}

pub const TABLE: &[PlasticityHardening] = &[
    PlasticityHardening { formula: "AISI_1020",   hollomon_k_pa: 7.45e8, hollomon_n: 0.20, jc_a_pa: 3.50e8, jc_b_pa: 2.75e8, jc_n: 0.36, jc_c: 0.022, jc_m: 1.00, jc_t_melt_k: 1811.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "AISI_1045",   hollomon_k_pa: 9.65e8, hollomon_n: 0.14, jc_a_pa: 5.07e8, jc_b_pa: 3.20e8, jc_n: 0.28, jc_c: 0.064, jc_m: 1.06, jc_t_melt_k: 1793.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "AISI_4340",   hollomon_k_pa: 2.65e9, hollomon_n: 0.07, jc_a_pa: 7.92e8, jc_b_pa: 5.10e8, jc_n: 0.26, jc_c: 0.014, jc_m: 1.03, jc_t_melt_k: 1793.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "AISI_304",    hollomon_k_pa: 1.30e9, hollomon_n: 0.45, jc_a_pa: 3.10e8, jc_b_pa: 1.00e9, jc_n: 0.65, jc_c: 0.070, jc_m: 1.00, jc_t_melt_k: 1673.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "Al_6061_T6",  hollomon_k_pa: 4.10e8, hollomon_n: 0.05, jc_a_pa: 3.24e8, jc_b_pa: 1.14e8, jc_n: 0.42, jc_c: 0.002, jc_m: 1.34, jc_t_melt_k: 925.0,  jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "Al_7075_T6",  hollomon_k_pa: 7.80e8, hollomon_n: 0.06, jc_a_pa: 5.20e8, jc_b_pa: 4.77e8, jc_n: 0.52, jc_c: 0.001, jc_m: 1.00, jc_t_melt_k: 893.0,  jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "Al_2024_T3",  hollomon_k_pa: 6.90e8, hollomon_n: 0.16, jc_a_pa: 3.69e8, jc_b_pa: 6.84e8, jc_n: 0.73, jc_c: 0.008, jc_m: 1.70, jc_t_melt_k: 775.0,  jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "Cu",          hollomon_k_pa: 5.30e8, hollomon_n: 0.54, jc_a_pa: 9.00e7, jc_b_pa: 2.92e8, jc_n: 0.31, jc_c: 0.025, jc_m: 1.09, jc_t_melt_k: 1356.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "Ti_6Al_4V",   hollomon_k_pa: 1.27e9, hollomon_n: 0.05, jc_a_pa: 8.62e8, jc_b_pa: 3.31e8, jc_n: 0.34, jc_c: 0.012, jc_m: 0.80, jc_t_melt_k: 1878.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "Inconel_718", hollomon_k_pa: 1.55e9, hollomon_n: 0.06, jc_a_pa: 1.24e9, jc_b_pa: 1.20e9, jc_n: 0.65, jc_c: 0.014, jc_m: 1.30, jc_t_melt_k: 1609.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "W",           hollomon_k_pa: 1.50e9, hollomon_n: 0.20, jc_a_pa: 1.51e9, jc_b_pa: 1.77e8, jc_n: 0.12, jc_c: 0.016, jc_m: 1.00, jc_t_melt_k: 3695.0, jc_eps_dot_ref: 1.0 },
    PlasticityHardening { formula: "Mg",          hollomon_k_pa: 4.50e8, hollomon_n: 0.14, jc_a_pa: 1.90e8, jc_b_pa: 2.55e8, jc_n: 0.40, jc_c: 0.013, jc_m: 1.50, jc_t_melt_k: 923.0,  jc_eps_dot_ref: 1.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static PlasticityHardening> {
    TABLE.iter().find(|p| p.formula == formula)
}
