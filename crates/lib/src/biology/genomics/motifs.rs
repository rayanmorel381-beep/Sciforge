pub fn pwm_score(pwm: &[Vec<f64>], sequence: &[u8]) -> f64 {
    let n = pwm.len().min(sequence.len());
    let mut score = 0.0;
    for i in 0..n {
        let idx = base_to_index(sequence[i]);
        if idx < pwm[i].len() {
            score += pwm[i][idx];
        }
    }
    score
}

pub fn pwm_scan(pwm: &[Vec<f64>], sequence: &[u8], threshold: f64) -> Vec<(usize, f64)> {
    let motif_len = pwm.len();
    let mut hits = Vec::new();
    if sequence.len() < motif_len {
        return hits;
    }
    for i in 0..=sequence.len() - motif_len {
        let score = pwm_score(pwm, &sequence[i..i + motif_len]);
        if score >= threshold {
            hits.push((i, score));
        }
    }
    hits
}

pub fn information_content(pwm: &[Vec<f64>]) -> Vec<f64> {
    pwm.iter()
        .map(|col| {
            let max_bits = 2.0_f64;
            let entropy: f64 = col
                .iter()
                .filter(|&&p| p > 0.0)
                .map(|&p| -p * p.log2())
                .sum();
            max_bits - entropy
        })
        .collect()
}

pub fn total_information(pwm: &[Vec<f64>]) -> f64 {
    information_content(pwm).iter().sum()
}

pub fn consensus_sequence(pwm: &[Vec<f64>]) -> String {
    let bases = [b'A', b'C', b'G', b'T'];
    pwm.iter()
        .map(|col| {
            let max_idx = col
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(0);
            if max_idx < bases.len() {
                bases[max_idx] as char
            } else {
                'N'
            }
        })
        .collect()
}

pub fn frequency_matrix(sequences: &[&[u8]], length: usize) -> Vec<Vec<f64>> {
    let n = sequences.len() as f64;
    let mut matrix = vec![vec![0.0; 4]; length];
    for seq in sequences {
        for (i, &base) in seq.iter().enumerate() {
            if i >= length {
                break;
            }
            let idx = base_to_index(base);
            if idx < 4 {
                matrix[i][idx] += 1.0;
            }
        }
    }
    for col in matrix.iter_mut() {
        for val in col.iter_mut() {
            *val /= n;
        }
    }
    matrix
}

fn base_to_index(base: u8) -> usize {
    match base {
        b'A' | b'a' => 0,
        b'C' | b'c' => 1,
        b'G' | b'g' => 2,
        b'T' | b't' => 3,
        _ => 4,
    }
}
