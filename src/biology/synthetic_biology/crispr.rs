pub fn crispr_on_target_score(gc_content: f64, position_scores: &[f64]) -> f64 {
    let pos_score: f64 = if position_scores.is_empty() {
        0.5
    } else {
        position_scores.iter().sum::<f64>() / position_scores.len() as f64
    };
    let gc_penalty = if !(0.4..=0.7).contains(&gc_content) {
        0.8
    } else {
        1.0
    };
    pos_score * gc_penalty
}

pub fn crispr_off_target_cfd(mismatches: &[(usize, f64)]) -> f64 {
    let mut score = 1.0;
    for &(_, penalty) in mismatches {
        score *= 1.0 - penalty;
    }
    score
}

pub fn cas9_cleavage_efficiency(
    grna_binding: f64,
    pam_strength: f64,
    chromatin_accessibility: f64,
) -> f64 {
    grna_binding * pam_strength * chromatin_accessibility
}

pub fn hdr_efficiency(
    template_conc: f64,
    homology_arm_length: f64,
    cell_cycle_s_fraction: f64,
) -> f64 {
    let template_factor = template_conc / (template_conc + 1.0);
    let arm_factor = (homology_arm_length / 500.0).min(1.0);
    template_factor * arm_factor * cell_cycle_s_fraction
}

pub fn nhej_indel_spectrum(cut_position: f64, microhomology_scores: &[f64]) -> Vec<f64> {
    let total: f64 = microhomology_scores.iter().map(|s| s.exp()).sum();
    microhomology_scores
        .iter()
        .map(|s| s.exp() / total * cut_position / cut_position)
        .collect()
}

pub fn base_editing_efficiency(
    c_to_t_window: bool,
    accessibility: f64,
    deaminase_activity: f64,
) -> f64 {
    if !c_to_t_window {
        return 0.0;
    }
    accessibility * deaminase_activity
}

pub fn prime_editing_efficiency(
    pbs_length: usize,
    rt_template_length: usize,
    nick_distance: usize,
) -> f64 {
    let pbs_factor = if (10..=15).contains(&pbs_length) {
        1.0
    } else {
        0.5
    };
    let rt_factor = if (10..=30).contains(&rt_template_length) {
        1.0
    } else {
        0.6
    };
    let nick_factor = if (40..=80).contains(&nick_distance) {
        1.0
    } else {
        0.7
    };
    pbs_factor * rt_factor * nick_factor * 0.5
}

pub fn guide_rna_folding_penalty(self_complementarity_score: f64) -> f64 {
    (-0.1 * self_complementarity_score).exp()
}

pub fn multiplex_editing_probability(efficiencies: &[f64]) -> f64 {
    let mut prob = 1.0;
    for &e in efficiencies {
        prob *= e;
    }
    prob
}

pub fn gene_drive_frequency(
    drive_efficiency: f64,
    fitness_cost: f64,
    generations: usize,
    initial_freq: f64,
) -> f64 {
    let mut freq = initial_freq;
    for _ in 0..generations {
        let homing = drive_efficiency * freq * (1.0 - freq);
        let selection = -fitness_cost * freq * freq;
        freq = (freq + homing + selection).clamp(0.0, 1.0);
    }
    freq
}
