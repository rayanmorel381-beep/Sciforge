pub fn cell_cycle_ode(
    cyclin: f64,
    cdk: f64,
    apc: f64,
    k_syn: f64,
    k_deg: f64,
    k_act: f64,
    k_inact: f64,
) -> (f64, f64, f64) {
    let dcyclin = k_syn - k_deg * apc * cyclin;
    let dcdk = k_act * cyclin * (1.0 - cdk) - k_inact * cdk;
    let dapc = k_act * cdk * (1.0 - apc) - k_inact * apc;
    (dcyclin, dcdk, dapc)
}

pub fn cell_cycle_simulate(
    cyclin0: f64,
    cdk0: f64,
    apc0: f64,
    k_syn: f64,
    k_deg: f64,
    k_act: f64,
    k_inact: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut cyclin, mut cdk, mut apc) = (cyclin0, cdk0, apc0);
    result.push((cyclin, cdk, apc));
    for _ in 0..steps {
        let (dc, dd, da) = cell_cycle_ode(cyclin, cdk, apc, k_syn, k_deg, k_act, k_inact);
        cyclin += dc * dt;
        cdk += dd * dt;
        apc += da * dt;
        cyclin = cyclin.max(0.0);
        cdk = cdk.clamp(0.0, 1.0);
        apc = apc.clamp(0.0, 1.0);
        result.push((cyclin, cdk, apc));
    }
    result
}

pub fn mitotic_index(dividing_cells: usize, total_cells: usize) -> f64 {
    if total_cells == 0 {
        return 0.0;
    }
    dividing_cells as f64 / total_cells as f64
}

pub fn cell_growth_logistic(n: f64, r: f64, k: f64, dt: f64, steps: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(steps + 1);
    let mut pop = n;
    result.push(pop);
    for _ in 0..steps {
        let dpop = r * pop * (1.0 - pop / k);
        pop += dpop * dt;
        pop = pop.max(0.0);
        result.push(pop);
    }
    result
}

pub fn g1_checkpoint(dna_damage: f64, p53_threshold: f64, rb_active: f64) -> bool {
    dna_damage < p53_threshold && rb_active > 0.5
}

pub fn g2_checkpoint(dna_damage: f64, repair_capacity: f64, cdk1_activity: f64) -> bool {
    dna_damage / repair_capacity.max(1e-10) < 1.0 && cdk1_activity > 0.5
}

pub fn spindle_assembly_checkpoint(unattached_kinetochores: usize) -> bool {
    unattached_kinetochores == 0
}

pub fn apoptosis_probability(dna_damage: f64, p53: f64, bcl2: f64, bax: f64) -> f64 {
    let pro_apoptotic = p53 * bax * dna_damage;
    let anti_apoptotic = bcl2;
    pro_apoptotic / (pro_apoptotic + anti_apoptotic + 1e-30)
}

pub fn cell_doubling_time(growth_rate: f64) -> f64 {
    if growth_rate <= 0.0 {
        return f64::INFINITY;
    }
    (2.0_f64).ln() / growth_rate
}

pub fn contact_inhibition(density: f64, max_density: f64, steepness: f64) -> f64 {
    1.0 / (1.0 + (steepness * (density / max_density - 0.8)).exp())
}

pub fn phase_duration(
    total_cycle_time: f64,
    g1_fraction: f64,
    s_fraction: f64,
    g2_fraction: f64,
) -> (f64, f64, f64, f64) {
    let m_fraction = 1.0 - g1_fraction - s_fraction - g2_fraction;
    (
        total_cycle_time * g1_fraction,
        total_cycle_time * s_fraction,
        total_cycle_time * g2_fraction,
        total_cycle_time * m_fraction.max(0.0),
    )
}

pub fn dna_damage_accumulation(
    damage: f64,
    production_rate: f64,
    repair_rate: f64,
    dt: f64,
) -> f64 {
    (damage + (production_rate - repair_rate * damage) * dt).max(0.0)
}

pub fn restriction_point(growth_factor: f64, threshold: f64, rb_phosphorylation: f64) -> bool {
    growth_factor > threshold && rb_phosphorylation > 0.5
}

pub fn cyclin_oscillator(cyclin: f64, cdk: f64, k_syn: f64, k_deg: f64) -> f64 {
    k_syn - k_deg * cyclin * cdk / (cyclin + 0.1)
}

pub fn cell_senescence_probability(
    telomere_length: f64,
    critical_length: f64,
    dna_damage: f64,
) -> f64 {
    let telomere_signal = if telomere_length < critical_length {
        1.0 - telomere_length / critical_length
    } else {
        0.0
    };
    (telomere_signal + dna_damage).min(1.0)
}

pub fn proliferation_index(s_phase: usize, g2m_phase: usize, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    (s_phase + g2m_phase) as f64 / total as f64
}

pub fn growth_fraction(proliferating: usize, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    proliferating as f64 / total as f64
}

pub fn cell_loss_factor(growth_rate: f64, doubling_time: f64) -> f64 {
    if doubling_time <= 0.0 {
        return 0.0;
    }
    1.0 - growth_rate * doubling_time / (2.0_f64).ln()
}

pub fn hayflick_limit(initial_telomere: f64, loss_per_division: f64, critical: f64) -> f64 {
    if loss_per_division <= 0.0 {
        return f64::INFINITY;
    }
    ((initial_telomere - critical) / loss_per_division).max(0.0)
}

pub fn quiescence_entry(
    growth_factor: f64,
    nutrient: f64,
    gf_threshold: f64,
    nutrient_threshold: f64,
) -> bool {
    growth_factor < gf_threshold || nutrient < nutrient_threshold
}
