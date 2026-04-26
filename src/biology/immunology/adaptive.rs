pub fn clonal_expansion(n0: f64, proliferation_rate: f64, death_rate: f64, t: f64) -> f64 {
    n0 * ((proliferation_rate - death_rate) * t).exp()
}

pub fn clonal_selection_dynamics(
    naive: f64,
    effector: f64,
    memory: f64,
    antigen: f64,
    k_activation: f64,
    k_prolif: f64,
    k_death: f64,
    k_memory: f64,
    k_clear: f64,
) -> (f64, f64, f64, f64) {
    let activation = k_activation * naive * antigen;
    let dnaive = -activation - 0.001 * memory;
    let deffector =
        k_prolif * effector * antigen + activation - k_death * effector - k_memory * effector;
    let dmemory = k_memory * effector;
    let dantigen = -k_clear * effector * antigen;
    (dnaive, deffector, dmemory, dantigen)
}

pub fn clonal_selection_simulate(
    naive0: f64,
    effector0: f64,
    memory0: f64,
    antigen0: f64,
    k_activation: f64,
    k_prolif: f64,
    k_death: f64,
    k_memory: f64,
    k_clear: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64, f64, f64)> {
    let mut result = Vec::with_capacity(steps + 1);
    let (mut n, mut e, mut m, mut ag) = (naive0, effector0, memory0, antigen0);
    result.push((n, e, m, ag));
    for _ in 0..steps {
        let (dn, de, dm, dag) = clonal_selection_dynamics(
            n,
            e,
            m,
            ag,
            k_activation,
            k_prolif,
            k_death,
            k_memory,
            k_clear,
        );
        n = (n + dn * dt).max(0.0);
        e = (e + de * dt).max(0.0);
        m = (m + dm * dt).max(0.0);
        ag = (ag + dag * dt).max(0.0);
        result.push((n, e, m, ag));
    }
    result
}

pub fn tcell_activation_threshold(signal: f64, costimulation: f64, threshold: f64) -> bool {
    signal * costimulation > threshold
}

pub fn cytokine_hill_response(cytokine: f64, ec50: f64, n: f64) -> f64 {
    let cn = cytokine.powf(n);
    cn / (ec50.powf(n) + cn)
}

pub fn treg_suppression(effector_rate: f64, treg: f64, k_supp: f64) -> f64 {
    effector_rate / (1.0 + k_supp * treg)
}

pub fn memory_recall_response(
    memory: f64,
    antigen: f64,
    k_recall: f64,
    fold_expansion: f64,
) -> f64 {
    memory * fold_expansion * antigen / (k_recall + antigen)
}
