use crate::constants::{
    DNA_DAMAGE_BASE_WEIGHT, DNA_DAMAGE_DSB_WEIGHT, DSB_YIELD_PER_GY, OXIDATIVE_DAMAGE_COEFF,
    OXIDATIVE_DAMAGE_O2_KM, SSB_YIELD_PER_GY,
};

pub fn dna_strand_break_probability(dose: f64, target_size: f64, repair_efficiency: f64) -> f64 {
    1.0 - (-dose * target_size * (1.0 - repair_efficiency)).exp()
}

pub fn base_excision_repair_rate(damage_sites: f64, enzyme_concentration: f64, km: f64) -> f64 {
    enzyme_concentration * damage_sites / (km + damage_sites)
}

pub fn misrepair_probability(damage_density: f64, complexity_factor: f64) -> f64 {
    1.0 - (-complexity_factor * damage_density).exp()
}

pub fn chromosome_aberration_yield(dose: f64, alpha: f64, beta: f64) -> f64 {
    alpha * dose + beta * dose * dose
}

pub fn lethal_aberration_fraction(aberrations: f64) -> f64 {
    1.0 - (-aberrations).exp()
}

pub fn mutation_frequency(dose: f64, spontaneous_rate: f64, induced_rate_per_gy: f64) -> f64 {
    spontaneous_rate + induced_rate_per_gy * dose
}

pub fn double_strand_break_yield(dose: f64, let_factor: f64) -> f64 {
    DSB_YIELD_PER_GY * dose * let_factor
}

pub fn nhej_repair_kinetics(
    breaks: f64,
    fast_rate: f64,
    slow_rate: f64,
    fast_fraction: f64,
    t: f64,
) -> f64 {
    breaks
        * (fast_fraction * (-fast_rate * t).exp() + (1.0 - fast_fraction) * (-slow_rate * t).exp())
}

pub fn homologous_recombination_probability(
    cell_cycle_s_g2_fraction: f64,
    sister_chromatid_available: bool,
) -> f64 {
    if sister_chromatid_available {
        cell_cycle_s_g2_fraction
    } else {
        0.0
    }
}

pub fn clustered_damage_probability(dose: f64, let_val: f64, target_radius: f64) -> f64 {
    let hits = dose * let_val * target_radius * target_radius;
    1.0 - (1.0 + hits) * (-hits).exp()
}

pub fn single_strand_break_yield(dose: f64) -> f64 {
    SSB_YIELD_PER_GY * dose
}

pub fn oxidative_base_damage_yield(dose: f64, oxygen_concentration: f64) -> f64 {
    OXIDATIVE_DAMAGE_COEFF * dose * oxygen_concentration
        / (oxygen_concentration + OXIDATIVE_DAMAGE_O2_KM)
}

pub fn dna_damage_complexity_score(ssb: f64, dsb: f64, base_damage: f64) -> f64 {
    ssb + DNA_DAMAGE_DSB_WEIGHT * dsb + DNA_DAMAGE_BASE_WEIGHT * base_damage
}

pub fn foci_formation_kinetics(dsb: f64, recruitment_rate: f64, t: f64) -> f64 {
    dsb * (1.0 - (-recruitment_rate * t).exp())
}

pub fn foci_resolution_kinetics(foci_max: f64, repair_rate: f64, t: f64) -> f64 {
    foci_max * (-repair_rate * t).exp()
}

pub fn micronucleus_formation(dose: f64, alpha_mn: f64, beta_mn: f64) -> f64 {
    alpha_mn * dose + beta_mn * dose * dose
}

pub fn comet_tail_moment(tail_length: f64, tail_dna_fraction: f64) -> f64 {
    tail_length * tail_dna_fraction
}

pub fn gamma_h2ax_signal(dsb: f64, spreading_factor: f64, background: f64) -> f64 {
    background + spreading_factor * dsb
}

pub fn repair_pathway_choice(dsb: f64, cell_cycle_phase: f64, brca_status: f64) -> (f64, f64) {
    let hr_fraction = cell_cycle_phase * brca_status;
    let nhej_fraction = 1.0 - hr_fraction;
    (dsb * nhej_fraction, dsb * hr_fraction)
}
