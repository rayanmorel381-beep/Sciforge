pub fn phred_to_probability(phred: f64) -> f64 {
    (10.0_f64).powf(-phred / 10.0)
}

pub fn probability_to_phred(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::INFINITY;
    }
    -10.0 * p.log10()
}

pub fn average_quality(qualities: &[u8]) -> f64 {
    if qualities.is_empty() {
        return 0.0;
    }
    let sum_prob: f64 = qualities
        .iter()
        .map(|&q| phred_to_probability(q as f64))
        .sum();
    let avg_prob = sum_prob / qualities.len() as f64;
    probability_to_phred(avg_prob)
}

pub fn quality_filter(qualities: &[u8], min_quality: u8, window: usize, min_fraction: f64) -> bool {
    if qualities.len() < window {
        return false;
    }
    let good_windows = qualities
        .windows(window)
        .filter(|w| {
            let avg: f64 = w.iter().map(|&q| q as f64).sum::<f64>() / w.len() as f64;
            avg >= min_quality as f64
        })
        .count();
    good_windows as f64 / (qualities.len() - window + 1) as f64 >= min_fraction
}

pub fn expected_errors(qualities: &[u8]) -> f64 {
    qualities
        .iter()
        .map(|&q| phred_to_probability(q as f64))
        .sum()
}

pub fn trim_quality(qualities: &[u8], min_quality: u8) -> usize {
    let mut best_end = qualities.len();
    let mut running_sum = 0.0;
    let mut best_sum = 0.0;
    for (i, &q) in qualities.iter().enumerate().rev() {
        running_sum += q as f64 - min_quality as f64;
        if running_sum >= best_sum {
            best_sum = running_sum;
            best_end = i;
        }
    }
    best_end
}

pub fn n50(lengths: &[usize]) -> usize {
    let mut sorted = lengths.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let total: usize = sorted.iter().sum();
    let half = total / 2;
    let mut cumulative = 0;
    for &len in &sorted {
        cumulative += len;
        if cumulative >= half {
            return len;
        }
    }
    0
}

pub fn gc_content(sequence: &[u8]) -> f64 {
    if sequence.is_empty() {
        return 0.0;
    }
    let gc = sequence
        .iter()
        .filter(|&&b| matches!(b, b'G' | b'g' | b'C' | b'c'))
        .count();
    gc as f64 / sequence.len() as f64
}

pub fn adapter_match_score(read: &[u8], adapter: &[u8]) -> usize {
    let mut best = 0;
    for start in 0..read.len() {
        let len = read.len() - start;
        let check = len.min(adapter.len());
        let matches = (0..check)
            .filter(|&i| read[start + i] == adapter[i])
            .count();
        if matches > best {
            best = matches;
        }
    }
    best
}

pub fn complexity_dust(sequence: &[u8], window: usize) -> f64 {
    if sequence.len() < window {
        return 0.0;
    }
    let mut max_score = 0.0;
    for w in sequence.windows(window) {
        let mut triplet_counts = [0u32; 64];
        for tri in w.windows(3) {
            let idx = ((tri[0] & 0x6) as usize) << 3
                | ((tri[1] & 0x6) as usize) << 1
                | (tri[2] & 0x6) as usize >> 1;
            triplet_counts[idx % 64] += 1;
        }
        let score: f64 = triplet_counts
            .iter()
            .map(|&c| c as f64 * (c as f64 - 1.0) / 2.0)
            .sum();
        if score > max_score {
            max_score = score;
        }
    }
    max_score
}

pub fn kmer_frequency(sequence: &[u8], k: usize) -> Vec<(Vec<u8>, usize)> {
    if sequence.len() < k || k == 0 {
        return Vec::new();
    }
    let mut counts: Vec<(Vec<u8>, usize)> = Vec::new();
    for window in sequence.windows(k) {
        if let Some(entry) = counts.iter_mut().find(|(kmer, _)| kmer == window) {
            entry.1 += 1;
        } else {
            counts.push((window.to_vec(), 1));
        }
    }
    counts
}

pub fn shannon_entropy_sequence(sequence: &[u8]) -> f64 {
    if sequence.is_empty() {
        return 0.0;
    }
    let mut counts = [0usize; 256];
    for &b in sequence {
        counts[b as usize] += 1;
    }
    let n = sequence.len() as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.ln()
        })
        .sum()
}

pub fn sliding_window_quality(qualities: &[u8], window: usize) -> Vec<f64> {
    if qualities.len() < window {
        return Vec::new();
    }
    qualities
        .windows(window)
        .map(|w| w.iter().map(|&q| q as f64).sum::<f64>() / w.len() as f64)
        .collect()
}

pub fn per_base_quality_distribution(quality_matrix: &[Vec<u8>]) -> Vec<(f64, f64)> {
    if quality_matrix.is_empty() {
        return Vec::new();
    }
    let max_len = quality_matrix.iter().map(|r| r.len()).max().unwrap_or(0);
    (0..max_len)
        .map(|pos| {
            let vals: Vec<f64> = quality_matrix
                .iter()
                .filter(|r| pos < r.len())
                .map(|r| r[pos] as f64)
                .collect();
            if vals.is_empty() {
                return (0.0, 0.0);
            }
            let mean = vals.iter().sum::<f64>() / vals.len() as f64;
            let var = vals.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / vals.len() as f64;
            (mean, var.sqrt())
        })
        .collect()
}

pub fn duplication_rate(total_reads: usize, unique_reads: usize) -> f64 {
    1.0 - unique_reads as f64 / total_reads.max(1) as f64
}

pub fn chimera_breakpoint_score(alignment_a: usize, alignment_b: usize, read_length: usize) -> f64 {
    let coverage = (alignment_a + alignment_b) as f64 / read_length.max(1) as f64;
    coverage.min(2.0)
}
