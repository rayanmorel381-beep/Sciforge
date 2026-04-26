pub fn histone_mark_occupancy(k_on: f64, k_off: f64) -> f64 {
    k_on / (k_on + k_off)
}

pub fn histone_mark_dynamics(
    occupancy: f64,
    k_on: f64,
    k_off: f64,
    k_spread: f64,
    neighbor_occ: f64,
) -> f64 {
    (k_on + k_spread * neighbor_occ) * (1.0 - occupancy) - k_off * occupancy
}

pub fn histone_spread_simulate(
    occupancies: &mut [f64],
    k_on: f64,
    k_off: f64,
    k_spread: f64,
    dt: f64,
    steps: usize,
) -> Vec<Vec<f64>> {
    let n = occupancies.len();
    let mut result = Vec::with_capacity(steps + 1);
    result.push(occupancies.to_vec());
    for _ in 0..steps {
        let old = occupancies.to_vec();
        for i in 0..n {
            let left = if i > 0 { old[i - 1] } else { 0.0 };
            let right = if i < n - 1 { old[i + 1] } else { 0.0 };
            let neighbor = (left + right) / 2.0;
            let d = histone_mark_dynamics(old[i], k_on, k_off, k_spread, neighbor);
            occupancies[i] = (old[i] + d * dt).clamp(0.0, 1.0);
        }
        result.push(occupancies.to_vec());
    }
    result
}

pub fn nucleosome_positioning_energy(
    dna_flexibility: &[f64],
    position: usize,
    wrap_length: usize,
) -> f64 {
    let end = (position + wrap_length).min(dna_flexibility.len());
    let mut energy = 0.0;
    for &flex in &dna_flexibility[position..end] {
        energy += 1.0 / flex.max(1e-30);
    }
    energy
}

pub fn chromatin_compaction_ratio(extended_length: f64, compacted_length: f64) -> f64 {
    extended_length / compacted_length.max(1e-30)
}

pub fn histone_acetylation_equilibrium(hat_activity: f64, hdac_activity: f64) -> f64 {
    hat_activity / (hat_activity + hdac_activity + 1e-30)
}

pub fn bivalent_domain_resolution(
    h3k4me3: f64,
    h3k27me3: f64,
    signal: f64,
    threshold: f64,
) -> (f64, f64) {
    if signal > threshold {
        (h3k4me3 + 0.1 * signal, h3k27me3 * 0.9)
    } else {
        (h3k4me3 * 0.9, h3k27me3 + 0.1 * (threshold - signal))
    }
}

pub fn chip_seq_enrichment(
    ip_reads: f64,
    input_reads: f64,
    ip_total: f64,
    input_total: f64,
) -> f64 {
    if input_reads < 1.0 || ip_total < 1.0 || input_total < 1.0 {
        return 0.0;
    }
    (ip_reads / ip_total) / (input_reads / input_total)
}

pub fn reader_writer_feedback(
    mark: f64,
    reader_affinity: f64,
    writer_rate: f64,
    eraser_rate: f64,
) -> f64 {
    let read_signal = reader_affinity * mark;
    writer_rate * read_signal / (1.0 + read_signal) - eraser_rate * mark
}

pub fn heterochromatin_spreading(
    marks: &mut [f64],
    spread_rate: f64,
    barrier_positions: &[usize],
    dt: f64,
) {
    let n = marks.len();
    let old = marks.to_vec();
    for i in 0..n {
        if barrier_positions.contains(&i) {
            continue;
        }
        let left = if i > 0 && !barrier_positions.contains(&(i - 1)) {
            old[i - 1]
        } else {
            0.0
        };
        let right = if i < n - 1 && !barrier_positions.contains(&(i + 1)) {
            old[i + 1]
        } else {
            0.0
        };
        let spread = spread_rate * ((left + right) / 2.0 - old[i]);
        marks[i] = (old[i] + spread * dt).clamp(0.0, 1.0);
    }
}
