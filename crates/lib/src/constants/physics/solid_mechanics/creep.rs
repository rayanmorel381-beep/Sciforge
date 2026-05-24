#[derive(Debug, Clone, Copy)]
pub struct Creep {
    pub formula: &'static str,
    pub norton_a: f64,
    pub norton_n: f64,
    pub activation_energy_q_j_per_mol: f64,
    pub larson_miller_c: f64,
    pub max_service_temp_k: f64,
}

pub const TABLE: &[Creep] = &[
    Creep { formula: "AISI_304",    norton_a: 1.0e-20, norton_n: 7.5,  activation_energy_q_j_per_mol: 280000.0, larson_miller_c: 20.0, max_service_temp_k: 1090.0 },
    Creep { formula: "AISI_316",    norton_a: 1.5e-20, norton_n: 7.0,  activation_energy_q_j_per_mol: 290000.0, larson_miller_c: 20.0, max_service_temp_k: 1090.0 },
    Creep { formula: "AISI_4140",   norton_a: 1.0e-22, norton_n: 6.0,  activation_energy_q_j_per_mol: 250000.0, larson_miller_c: 20.0, max_service_temp_k: 800.0 },
    Creep { formula: "AISI_P91",    norton_a: 5.0e-23, norton_n: 8.0,  activation_energy_q_j_per_mol: 320000.0, larson_miller_c: 30.0, max_service_temp_k: 920.0 },
    Creep { formula: "Inconel_718", norton_a: 1.0e-25, norton_n: 6.5,  activation_energy_q_j_per_mol: 360000.0, larson_miller_c: 20.0, max_service_temp_k: 920.0 },
    Creep { formula: "Inconel_625", norton_a: 5.0e-25, norton_n: 5.0,  activation_energy_q_j_per_mol: 350000.0, larson_miller_c: 20.0, max_service_temp_k: 1250.0 },
    Creep { formula: "Inconel_738", norton_a: 1.0e-26, norton_n: 5.5,  activation_energy_q_j_per_mol: 380000.0, larson_miller_c: 20.0, max_service_temp_k: 1320.0 },
    Creep { formula: "CMSX_4",      norton_a: 1.0e-27, norton_n: 6.0,  activation_energy_q_j_per_mol: 420000.0, larson_miller_c: 20.0, max_service_temp_k: 1370.0 },
    Creep { formula: "Ti_6Al_4V",   norton_a: 5.0e-22, norton_n: 4.5,  activation_energy_q_j_per_mol: 240000.0, larson_miller_c: 20.0, max_service_temp_k: 670.0 },
    Creep { formula: "Al_6061_T6",  norton_a: 1.0e-15, norton_n: 4.0,  activation_energy_q_j_per_mol: 145000.0, larson_miller_c: 20.0, max_service_temp_k: 470.0 },
    Creep { formula: "Cu",          norton_a: 1.0e-18, norton_n: 4.8,  activation_energy_q_j_per_mol: 197000.0, larson_miller_c: 20.0, max_service_temp_k: 720.0 },
    Creep { formula: "W",           norton_a: 1.0e-26, norton_n: 4.0,  activation_energy_q_j_per_mol: 600000.0, larson_miller_c: 20.0, max_service_temp_k: 2200.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static Creep> {
    TABLE.iter().find(|c| c.formula == formula)
}
