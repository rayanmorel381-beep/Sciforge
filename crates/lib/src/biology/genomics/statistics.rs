pub fn kmer_count(sequence: &[u8], k: usize) -> Vec<(Vec<u8>, usize)> {
    if k == 0 || sequence.len() < k {
        return Vec::new();
    }
    let mut counts: Vec<(Vec<u8>, usize)> = Vec::new();
    for window in sequence.windows(k) {
        let kmer = window.to_vec();
        let mut found = false;
        for entry in counts.iter_mut() {
            if entry.0 == kmer {
                entry.1 += 1;
                found = true;
                break;
            }
        }
        if !found {
            counts.push((kmer, 1));
        }
    }
    counts
}

pub fn gc_skew(sequence: &[u8], window: usize) -> Vec<f64> {
    if window == 0 || sequence.len() < window {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(sequence.len() - window + 1);
    for i in 0..=sequence.len() - window {
        let w = &sequence[i..i + window];
        let mut g = 0.0_f64;
        let mut c = 0.0_f64;
        for &b in w {
            match b {
                b'G' | b'g' => g += 1.0,
                b'C' | b'c' => c += 1.0,
                _ => {}
            }
        }
        let total = g + c;
        if total > 0.0 {
            result.push((g - c) / total);
        } else {
            result.push(0.0);
        }
    }
    result
}

pub fn cpg_observed_expected(sequence: &[u8]) -> f64 {
    if sequence.len() < 2 {
        return 0.0;
    }
    let n = sequence.len() as f64;
    let mut cpg_count = 0.0;
    let mut c_count = 0.0;
    let mut g_count = 0.0;
    for i in 0..sequence.len() {
        match sequence[i] {
            b'C' | b'c' => {
                c_count += 1.0;
                if i + 1 < sequence.len() && (sequence[i + 1] == b'G' || sequence[i + 1] == b'g') {
                    cpg_count += 1.0;
                }
            }
            b'G' | b'g' => g_count += 1.0,
            _ => {}
        }
    }
    let expected = (c_count * g_count) / n;
    if expected < 1e-30 {
        return 0.0;
    }
    cpg_count / expected
}

pub fn linguistic_complexity(sequence: &[u8]) -> f64 {
    let n = sequence.len();
    if n == 0 {
        return 0.0;
    }
    let mut total_observed = 0usize;
    let mut total_possible = 0usize;
    for k in 1..=n {
        let possible = 4_usize.pow(k as u32).min(n - k + 1);
        let observed = count_unique_kmers(sequence, k);
        total_observed += observed;
        total_possible += possible;
    }
    if total_possible == 0 {
        return 0.0;
    }
    total_observed as f64 / total_possible as f64
}

fn count_unique_kmers(sequence: &[u8], k: usize) -> usize {
    if k == 0 || sequence.len() < k {
        return 0;
    }
    let mut uniques: Vec<Vec<u8>> = Vec::new();
    for window in sequence.windows(k) {
        let w = window.to_vec();
        if !uniques.iter().any(|u| u == &w) {
            uniques.push(w);
        }
    }
    uniques.len()
}

pub fn at_content(sequence: &[u8]) -> f64 {
    if sequence.is_empty() {
        return 0.0;
    }
    let at: usize = sequence
        .iter()
        .filter(|&&b| matches!(b, b'A' | b'a' | b'T' | b't'))
        .count();
    at as f64 / sequence.len() as f64
}

pub fn dinucleotide_frequency(sequence: &[u8]) -> Vec<(u8, u8, f64)> {
    if sequence.len() < 2 {
        return Vec::new();
    }
    let mut counts = [[0usize; 4]; 4];
    let total = (sequence.len() - 1) as f64;
    for w in sequence.windows(2) {
        let i = match w[0] {
            b'A' | b'a' => 0,
            b'T' | b't' => 1,
            b'G' | b'g' => 2,
            b'C' | b'c' => 3,
            _ => continue,
        };
        let j = match w[1] {
            b'A' | b'a' => 0,
            b'T' | b't' => 1,
            b'G' | b'g' => 2,
            b'C' | b'c' => 3,
            _ => continue,
        };
        counts[i][j] += 1;
    }
    let bases = [b'A', b'T', b'G', b'C'];
    let mut result = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            if counts[i][j] > 0 {
                result.push((bases[i], bases[j], counts[i][j] as f64 / total));
            }
        }
    }
    result
}

pub fn sequence_entropy(sequence: &[u8]) -> f64 {
    let n = sequence.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mut counts = [0usize; 4];
    for &b in sequence {
        match b {
            b'A' | b'a' => counts[0] += 1,
            b'T' | b't' => counts[1] += 1,
            b'G' | b'g' => counts[2] += 1,
            b'C' | b'c' => counts[3] += 1,
            _ => {}
        }
    }
    let mut entropy = 0.0;
    for &c in &counts {
        if c > 0 {
            let p = c as f64 / n;
            entropy -= p * p.log2();
        }
    }
    entropy
}

pub fn transition_transversion(sequence_a: &[u8], sequence_b: &[u8]) -> f64 {
    let mut transitions = 0usize;
    let mut transversions = 0usize;
    for (&a, &b) in sequence_a.iter().zip(sequence_b.iter()) {
        if a == b {
            continue;
        }
        let is_transition = matches!(
            (a, b),
            (b'A', b'G')
                | (b'G', b'A')
                | (b'C', b'T')
                | (b'T', b'C')
                | (b'a', b'g')
                | (b'g', b'a')
                | (b'c', b't')
                | (b't', b'c')
        );
        if is_transition {
            transitions += 1;
        } else {
            transversions += 1;
        }
    }
    transitions as f64 / transversions.max(1) as f64
}
