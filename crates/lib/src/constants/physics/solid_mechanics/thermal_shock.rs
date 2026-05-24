#[derive(Debug, Clone, Copy)]
pub struct ThermalShock {
    pub formula: &'static str,
    pub r_first_pa_k: f64,
    pub r_prime_w_per_m: f64,
    pub r_prime_prime_prime_k: f64,
    pub max_delta_t_k: f64,
}

pub const TABLE: &[ThermalShock] = &[
    ThermalShock { formula: "Al2O3",      r_first_pa_k: 110.0, r_prime_w_per_m: 3960.0,  r_prime_prime_prime_k: 0.31,  max_delta_t_k: 200.0 },
    ThermalShock { formula: "SiC",        r_first_pa_k: 270.0, r_prime_w_per_m: 32400.0, r_prime_prime_prime_k: 1.04,  max_delta_t_k: 300.0 },
    ThermalShock { formula: "Si3N4",      r_first_pa_k: 580.0, r_prime_w_per_m: 17400.0, r_prime_prime_prime_k: 0.35,  max_delta_t_k: 600.0 },
    ThermalShock { formula: "ZrO2",       r_first_pa_k: 320.0, r_prime_w_per_m: 800.0,   r_prime_prime_prime_k: 0.32,  max_delta_t_k: 500.0 },
    ThermalShock { formula: "MgO",        r_first_pa_k: 80.0,  r_prime_w_per_m: 4000.0,  r_prime_prime_prime_k: 0.25,  max_delta_t_k: 100.0 },
    ThermalShock { formula: "B4C",        r_first_pa_k: 90.0,  r_prime_w_per_m: 2700.0,  r_prime_prime_prime_k: 0.45,  max_delta_t_k: 150.0 },
    ThermalShock { formula: "WC",         r_first_pa_k: 350.0, r_prime_w_per_m: 35000.0, r_prime_prime_prime_k: 0.50,  max_delta_t_k: 400.0 },
    ThermalShock { formula: "glass_soda", r_first_pa_k: 100.0, r_prime_w_per_m: 100.0,   r_prime_prime_prime_k: 0.05,  max_delta_t_k: 80.0 },
    ThermalShock { formula: "BK7",        r_first_pa_k: 130.0, r_prime_w_per_m: 130.0,   r_prime_prime_prime_k: 0.06,  max_delta_t_k: 100.0 },
    ThermalShock { formula: "SiO2",       r_first_pa_k: 1100.0,r_prime_w_per_m: 1500.0,  r_prime_prime_prime_k: 0.50,  max_delta_t_k: 1000.0 },
    ThermalShock { formula: "graphite",   r_first_pa_k: 200.0, r_prime_w_per_m: 24000.0, r_prime_prime_prime_k: 1.50,  max_delta_t_k: 1500.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static ThermalShock> {
    TABLE.iter().find(|t| t.formula == formula)
}
