pub fn mutation_rate_per_replication(
    mutations_observed: f64,
    genome_length: f64,
    replications: f64,
) -> f64 {
    mutations_observed / (genome_length * replications)
}

pub fn quasispecies_fitness(master_fitness: f64, mutation_rate: f64, genome_length: f64) -> f64 {
    master_fitness * (1.0 - mutation_rate).powf(genome_length)
}

pub fn error_threshold(genome_length: f64, superiority: f64) -> f64 {
    1.0 - (1.0 / superiority).powf(1.0 / genome_length)
}

pub fn antigenic_distance(epitope_a: &[f64], epitope_b: &[f64]) -> f64 {
    epitope_a
        .iter()
        .zip(epitope_b.iter())
        .map(|(&a, &b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}

pub fn recombination_probability(co_infection_rate: f64, recombination_rate: f64) -> f64 {
    co_infection_rate * recombination_rate
}

pub fn viral_diversity_shannon(frequencies: &[f64]) -> f64 {
    -frequencies
        .iter()
        .filter(|&&f| f > 0.0)
        .map(|&f| f * f.ln())
        .sum::<f64>()
}

pub fn selection_coefficient_viral(fitness_mutant: f64, fitness_wildtype: f64) -> f64 {
    (fitness_mutant - fitness_wildtype) / fitness_wildtype
}

pub fn molecular_clock_substitutions(rate: f64, time: f64) -> f64 {
    rate * time
}

pub fn coalescent_time_two_lineages(ne: f64) -> f64 {
    ne
}

pub fn expected_pairwise_differences(theta: f64) -> f64 {
    theta
}

pub fn reassortment_probability(segments: u32, co_infection_rate: f64) -> f64 {
    co_infection_rate * (1.0 - (0.5_f64).powi(segments as i32 - 1))
}

pub fn immune_escape_rate(mutation_rate: f64, epitopes: f64, fitness_cost: f64) -> f64 {
    mutation_rate * epitopes * (1.0 - fitness_cost)
}

pub fn antigenic_cartography_distance(titer_ref: f64, titer_cross: f64) -> f64 {
    (titer_ref / titer_cross.max(1e-30)).log2().max(0.0)
}

pub fn phylogenetic_diversity(branch_lengths: &[f64]) -> f64 {
    branch_lengths.iter().sum()
}

pub fn dn_ds_ratio(nonsynonymous: f64, synonymous: f64, sites_n: f64, sites_s: f64) -> f64 {
    (nonsynonymous / sites_n) / (synonymous / sites_s).max(1e-30)
}

pub fn bottleneck_drift(allele_freq: f64, bottleneck_size: f64) -> f64 {
    allele_freq * (1.0 - allele_freq) / (2.0 * bottleneck_size)
}

pub fn lethal_mutagenesis_threshold(replication_fidelity: f64, genome_length: f64) -> f64 {
    1.0 - replication_fidelity.powf(genome_length)
}

pub fn fitness_landscape_epistasis(w_ab: f64, w_a: f64, w_b: f64, w_wt: f64) -> f64 {
    (w_ab * w_wt) / (w_a * w_b).max(1e-30) - 1.0
}

pub fn zoonotic_spillover_rate(
    contact_rate: f64,
    cross_species_infectivity: f64,
    prevalence: f64,
) -> f64 {
    contact_rate * cross_species_infectivity * prevalence
}

pub fn antigenic_drift_rate(substitution_rate: f64, epitope_fraction: f64) -> f64 {
    substitution_rate * epitope_fraction
}
