#[derive(Debug, Clone, Copy)]
pub struct Fatigue {
    pub formula: &'static str,
    pub sigma_f_prime_pa: f64,
    pub basquin_b: f64,
    pub epsilon_f_prime: f64,
    pub coffin_manson_c: f64,
    pub endurance_limit_pa: f64,
    pub ultimate_pa: f64,
}

pub const TABLE: &[Fatigue] = &[
    Fatigue { formula: "AISI_1020",   sigma_f_prime_pa: 8.95e8,  basquin_b: -0.12,  epsilon_f_prime: 0.41,  coffin_manson_c: -0.51, endurance_limit_pa: 2.20e8, ultimate_pa: 4.50e8 },
    Fatigue { formula: "AISI_1045",   sigma_f_prime_pa: 1.23e9,  basquin_b: -0.095, epsilon_f_prime: 1.04,  coffin_manson_c: -0.66, endurance_limit_pa: 3.10e8, ultimate_pa: 6.25e8 },
    Fatigue { formula: "AISI_4140",   sigma_f_prime_pa: 1.94e9,  basquin_b: -0.083, epsilon_f_prime: 0.84,  coffin_manson_c: -0.62, endurance_limit_pa: 5.10e8, ultimate_pa: 1.02e9 },
    Fatigue { formula: "AISI_304",    sigma_f_prime_pa: 1.00e9,  basquin_b: -0.10,  epsilon_f_prime: 0.40,  coffin_manson_c: -0.50, endurance_limit_pa: 2.40e8, ultimate_pa: 5.05e8 },
    Fatigue { formula: "AISI_316",    sigma_f_prime_pa: 1.05e9,  basquin_b: -0.10,  epsilon_f_prime: 0.40,  coffin_manson_c: -0.50, endurance_limit_pa: 2.50e8, ultimate_pa: 5.15e8 },
    Fatigue { formula: "Al_6061_T6",  sigma_f_prime_pa: 7.43e8,  basquin_b: -0.106, epsilon_f_prime: 1.95,  coffin_manson_c: -0.83, endurance_limit_pa: 9.60e7, ultimate_pa: 3.10e8 },
    Fatigue { formula: "Al_7075_T6",  sigma_f_prime_pa: 1.47e9,  basquin_b: -0.143, epsilon_f_prime: 0.183, coffin_manson_c: -0.628,endurance_limit_pa: 1.59e8, ultimate_pa: 5.72e8 },
    Fatigue { formula: "Al_2024_T3",  sigma_f_prime_pa: 1.10e9,  basquin_b: -0.124, epsilon_f_prime: 0.22,  coffin_manson_c: -0.59, endurance_limit_pa: 1.40e8, ultimate_pa: 4.83e8 },
    Fatigue { formula: "Ti_6Al_4V",   sigma_f_prime_pa: 2.03e9,  basquin_b: -0.104, epsilon_f_prime: 0.841, coffin_manson_c: -0.69, endurance_limit_pa: 5.10e8, ultimate_pa: 9.50e8 },
    Fatigue { formula: "Inconel_718", sigma_f_prime_pa: 1.96e9,  basquin_b: -0.07,  epsilon_f_prime: 1.00,  coffin_manson_c: -0.65, endurance_limit_pa: 4.50e8, ultimate_pa: 1.24e9 },
    Fatigue { formula: "Cu",          sigma_f_prime_pa: 4.20e8,  basquin_b: -0.11,  epsilon_f_prime: 0.50,  coffin_manson_c: -0.54, endurance_limit_pa: 7.50e7, ultimate_pa: 2.20e8 },
    Fatigue { formula: "Cu_brass",    sigma_f_prime_pa: 5.50e8,  basquin_b: -0.10,  epsilon_f_prime: 0.42,  coffin_manson_c: -0.50, endurance_limit_pa: 1.10e8, ultimate_pa: 3.30e8 },
];

pub fn by_formula(formula: &str) -> Option<&'static Fatigue> {
    TABLE.iter().find(|f| f.formula == formula)
}
