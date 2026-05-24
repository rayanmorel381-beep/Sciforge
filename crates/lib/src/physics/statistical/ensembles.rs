use crate::constants::K_B;

pub fn boltzmann_entropy(microstates: f64) -> f64 {
    K_B * microstates.ln()
}

pub fn helmholtz_free_energy(internal_energy_j: f64, t_k: f64, entropy_j_per_k: f64) -> f64 {
    internal_energy_j - t_k * entropy_j_per_k
}

pub fn gibbs_free_energy(
    internal_energy_j: f64,
    pressure_pa: f64,
    volume_m3: f64,
    t_k: f64,
    entropy_j_per_k: f64,
) -> f64 {
    internal_energy_j + pressure_pa * volume_m3 - t_k * entropy_j_per_k
}

pub fn canonical_partition_function_two_level(
    energy_gap_j: f64,
    t_k: f64,
    degeneracy_ground: f64,
    degeneracy_excited: f64,
) -> f64 {
    degeneracy_ground + degeneracy_excited * (-energy_gap_j / (K_B * t_k)).exp()
}

pub fn average_energy_two_level(
    energy_gap_j: f64,
    t_k: f64,
    degeneracy_ground: f64,
    degeneracy_excited: f64,
) -> f64 {
    let z = canonical_partition_function_two_level(energy_gap_j, t_k, degeneracy_ground, degeneracy_excited);
    degeneracy_excited * energy_gap_j * (-energy_gap_j / (K_B * t_k)).exp() / z
}

pub fn grand_potential(t_k: f64, partition_function: f64) -> f64 {
    -K_B * t_k * partition_function.ln()
}
