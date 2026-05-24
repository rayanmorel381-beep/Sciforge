pub fn ros_production_rate(metabolic_rate: f64, coupling_efficiency: f64) -> f64 {
    metabolic_rate * (1.0 - coupling_efficiency)
}

pub fn antioxidant_capacity(
    sod: f64,
    catalase: f64,
    glutathione: f64,
    k_sod: f64,
    k_cat: f64,
    k_gsh: f64,
) -> f64 {
    sod * k_sod + catalase * k_cat + glutathione * k_gsh
}

pub fn oxidative_damage_rate(ros_level: f64, antioxidant: f64) -> f64 {
    (ros_level - antioxidant).max(0.0)
}

pub fn lipid_peroxidation(
    pufa_concentration: f64,
    ros_level: f64,
    k_initiation: f64,
    k_propagation: f64,
) -> f64 {
    k_initiation * ros_level * pufa_concentration + k_propagation * pufa_concentration
}

pub fn protein_carbonylation(protein_conc: f64, ros_level: f64, rate_constant: f64) -> f64 {
    rate_constant * protein_conc * ros_level
}

pub fn dna_8oxog_formation(ros_level: f64, guanine_sites: f64, k_oxidation: f64) -> f64 {
    k_oxidation * ros_level * guanine_sites
}

pub fn mitochondrial_ros_vicious_cycle(
    damage: f64,
    ros_base: f64,
    amplification: f64,
    repair_rate: f64,
    dt: f64,
) -> f64 {
    let ros = ros_base * (1.0 + amplification * damage);
    (damage + (ros - repair_rate * damage) * dt).max(0.0)
}

pub fn glutathione_ratio(gsh: f64, gssg: f64) -> f64 {
    gsh / gssg.max(1e-30)
}

pub fn fenton_reaction_rate(fe2: f64, h2o2: f64, k_fenton: f64) -> f64 {
    k_fenton * fe2 * h2o2
}

pub fn nrf2_response(ros_level: f64, keap1: f64, k_activation: f64) -> f64 {
    let free_nrf2 = ros_level / (keap1 + ros_level);
    k_activation * free_nrf2
}

pub fn carbonyl_stress(methylglyoxal: f64, glyoxalase: f64, km: f64) -> f64 {
    methylglyoxal - glyoxalase * methylglyoxal / (km + methylglyoxal)
}

pub fn oxidative_stress_index(total_oxidant: f64, total_antioxidant: f64) -> f64 {
    total_oxidant / total_antioxidant.max(1e-30) * 100.0
}
