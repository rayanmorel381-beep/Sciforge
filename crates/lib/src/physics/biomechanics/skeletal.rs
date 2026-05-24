pub fn bone_stress(force_n: f64, cross_section_m2: f64) -> f64 {
    force_n / cross_section_m2
}

pub fn euler_bernoulli_long_bone_deflection(
    load_n: f64,
    length_m: f64,
    youngs_modulus_pa: f64,
    second_moment_area_m4: f64,
) -> f64 {
    load_n * length_m.powi(3) / (3.0 * youngs_modulus_pa * second_moment_area_m4)
}

pub fn joint_reaction_force(
    body_weight_n: f64,
    moment_arm_external_m: f64,
    moment_arm_muscle_m: f64,
) -> f64 {
    body_weight_n * (1.0 + moment_arm_external_m / moment_arm_muscle_m)
}

pub fn cortical_bone_failure_strain_axial() -> f64 {
    0.0073
}

pub fn cancellous_bone_modulus_density(
    apparent_density_kg_per_m3: f64,
    coefficient_pa_per_density2: f64,
) -> f64 {
    coefficient_pa_per_density2 * apparent_density_kg_per_m3 * apparent_density_kg_per_m3
}
