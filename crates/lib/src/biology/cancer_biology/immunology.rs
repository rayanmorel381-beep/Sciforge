pub fn tumor_immune_ode(
    tumor: f64,
    immune: f64,
    growth_rate: f64,
    carrying_capacity: f64,
    kill_rate: f64,
    stimulation: f64,
    decay_rate: f64,
) -> (f64, f64) {
    let dt = growth_rate * tumor * (1.0 - tumor / carrying_capacity) - kill_rate * tumor * immune;
    let di = stimulation * tumor * immune / (tumor + 1.0) - decay_rate * immune;
    (dt, di)
}

pub fn tumor_immune_simulate(
    tumor0: f64,
    immune0: f64,
    growth_rate: f64,
    carrying_capacity: f64,
    kill_rate: f64,
    stimulation: f64,
    decay_rate: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut t, mut i) = (tumor0, immune0);
    result.push((t, i));
    for _ in 0..steps {
        let (dtu, dim) = tumor_immune_ode(
            t,
            i,
            growth_rate,
            carrying_capacity,
            kill_rate,
            stimulation,
            decay_rate,
        );
        t = (t + dtu * dt).max(0.0);
        i = (i + dim * dt).max(0.0);
        result.push((t, i));
    }
    result
}

pub fn immunoediting_escape(
    immunogenic_clones: f64,
    escape_mutation_rate: f64,
    immune_pressure: f64,
) -> f64 {
    immunogenic_clones * escape_mutation_rate * immune_pressure
}

pub fn checkpoint_blockade_effect(
    baseline_kill: f64,
    pd1_inhibition: f64,
    ctla4_inhibition: f64,
) -> f64 {
    baseline_kill * (1.0 + pd1_inhibition) * (1.0 + ctla4_inhibition)
}

pub fn car_t_cell_expansion(
    initial_cells: f64,
    antigen_density: f64,
    expansion_rate: f64,
    t: f64,
) -> f64 {
    initial_cells * (expansion_rate * antigen_density * t).exp()
}

pub fn cytokine_release_syndrome(
    activated_cells: f64,
    cytokine_per_cell: f64,
    clearance_rate: f64,
    t: f64,
) -> f64 {
    let production = activated_cells * cytokine_per_cell;
    production / clearance_rate * (1.0 - (-clearance_rate * t).exp())
}

pub fn tumor_neoantigen_fitness(
    binding_affinity: f64,
    expression_level: f64,
    clonality: f64,
) -> f64 {
    binding_affinity * expression_level * clonality
}

pub fn abscopal_effect(
    local_dose: f64,
    immune_activation: f64,
    distant_tumor: f64,
    sensitivity: f64,
) -> f64 {
    distant_tumor * (-sensitivity * local_dose * immune_activation).exp()
}
