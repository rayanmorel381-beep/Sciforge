pub fn antigen_antibody_binding(ab: f64, ag: f64, ka: f64) -> f64 {
    ka * ab * ag / (1.0 + ka * ag)
}

pub fn affinity_maturation(kd_initial: f64, mutation_rate: f64, generations: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(generations + 1);
    let mut kd = kd_initial;
    result.push(kd);
    for _ in 0..generations {
        kd *= 1.0 - mutation_rate * 0.1;
        kd = kd.max(1e-15);
        result.push(kd);
    }
    result
}

pub fn avidity(kd_monovalent: f64, n_sites: usize) -> f64 {
    kd_monovalent / n_sites as f64
}

pub fn neutralization_curve(dose: f64, ic50: f64, n: f64) -> f64 {
    let dn = dose.powf(n);
    dn / (ic50.powf(n) + dn)
}

pub fn complement_cascade(c0: f64, rate: f64, t: f64) -> f64 {
    c0 * (1.0 - (-rate * t).exp())
}

pub fn toll_like_receptor_activation(pamp: f64, receptor_density: f64, kd: f64) -> f64 {
    receptor_density * pamp / (kd + pamp)
}

pub fn phagocytosis_rate(pathogen: f64, phagocyte: f64, k_phag: f64, saturation: f64) -> f64 {
    k_phag * phagocyte * pathogen / (saturation + pathogen)
}

pub fn nk_cell_killing(
    target_mhc: f64,
    mhc_threshold: f64,
    activating_ligand: f64,
    kill_rate: f64,
) -> f64 {
    let inhibition = if target_mhc > mhc_threshold {
        0.0
    } else {
        1.0 - target_mhc / mhc_threshold
    };
    kill_rate * inhibition * activating_ligand
}

pub fn cytokine_cascade(
    initial_cytokines: &[f64],
    amplification: &[Vec<f64>],
    steps: usize,
) -> Vec<Vec<f64>> {
    let n = initial_cytokines.len();
    let mut result = Vec::with_capacity(steps + 1);
    let mut state = initial_cytokines.to_vec();
    result.push(state.clone());
    for _ in 0..steps {
        let mut next = vec![0.0; n];
        for (i, ni) in next.iter_mut().enumerate() {
            for (&sj, &aij) in state.iter().zip(amplification[i].iter()) {
                *ni += aij * sj;
            }
            *ni = ni.min(1e6);
        }
        state = next;
        result.push(state.clone());
    }
    result
}

pub fn inflammasome_activation(damp: f64, signal2: f64, threshold: f64, nlrp3: f64) -> f64 {
    if damp < threshold || signal2 < threshold {
        return 0.0;
    }
    nlrp3 * (damp - threshold) * (signal2 - threshold)
}
