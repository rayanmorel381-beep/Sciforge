pub fn capsid_triangulation_subunits(t_number: u32) -> u32 {
    60 * t_number
}

pub fn capsid_radius_from_subunits(n_subunits: f64, subunit_area: f64) -> f64 {
    (n_subunits * subunit_area / (4.0 * std::f64::consts::PI)).sqrt()
}

pub fn genome_packaging_density(genome_length_bp: f64, capsid_volume_nm3: f64) -> f64 {
    genome_length_bp / capsid_volume_nm3
}

pub fn viral_burst_size(total_virions: f64, infected_cells: f64) -> f64 {
    if infected_cells.abs() < 1e-15 {
        return 0.0;
    }
    total_virions / infected_cells
}

pub fn viral_decay_rate(initial_titer: f64, final_titer: f64, time: f64) -> f64 {
    if time.abs() < 1e-15 || final_titer <= 0.0 || initial_titer <= 0.0 {
        return 0.0;
    }
    (initial_titer.ln() - final_titer.ln()) / time
}

pub fn mutation_rate_per_site(mutations: f64, genome_length: f64, replications: f64) -> f64 {
    if genome_length * replications < 1e-15 {
        return 0.0;
    }
    mutations / (genome_length * replications)
}

pub fn quasispecies_error_threshold(genome_length: f64, mu_per_site: f64) -> f64 {
    (1.0 - mu_per_site).powf(genome_length)
}

pub fn receptor_binding_affinity(kon: f64, koff: f64) -> f64 {
    if kon.abs() < 1e-15 {
        return f64::INFINITY;
    }
    koff / kon
}

pub fn plaque_forming_units(plaques: f64, dilution_factor: f64, volume_ml: f64) -> f64 {
    if volume_ml.abs() < 1e-15 {
        return 0.0;
    }
    plaques / (dilution_factor * volume_ml)
}

pub fn epitope_distance(mismatches: f64, total_epitope_sites: f64) -> f64 {
    if total_epitope_sites.abs() < 1e-15 {
        return 0.0;
    }
    mismatches / total_epitope_sites
}
