pub fn smith_waterman_score(
    seq1: &[u8],
    seq2: &[u8],
    match_score: i32,
    mismatch: i32,
    gap: i32,
) -> i32 {
    let m = seq1.len();
    let n = seq2.len();
    let mut matrix = vec![vec![0i32; n + 1]; m + 1];
    let mut max_score = 0;
    for i in 1..=m {
        for j in 1..=n {
            let s = if seq1[i - 1] == seq2[j - 1] {
                match_score
            } else {
                mismatch
            };
            let diag = matrix[i - 1][j - 1] + s;
            let up = matrix[i - 1][j] + gap;
            let left = matrix[i][j - 1] + gap;
            matrix[i][j] = 0_i32.max(diag).max(up).max(left);
            if matrix[i][j] > max_score {
                max_score = matrix[i][j];
            }
        }
    }
    max_score
}

pub fn needleman_wunsch_score(
    seq1: &[u8],
    seq2: &[u8],
    match_score: i32,
    mismatch: i32,
    gap: i32,
) -> i32 {
    let m = seq1.len();
    let n = seq2.len();
    let mut matrix = vec![vec![0i32; n + 1]; m + 1];
    for (i, row) in matrix.iter_mut().enumerate() {
        row[0] = i as i32 * gap;
    }
    for (j, val) in matrix[0].iter_mut().enumerate() {
        *val = j as i32 * gap;
    }
    for i in 1..=m {
        for j in 1..=n {
            let s = if seq1[i - 1] == seq2[j - 1] {
                match_score
            } else {
                mismatch
            };
            let diag = matrix[i - 1][j - 1] + s;
            let up = matrix[i - 1][j] + gap;
            let left = matrix[i][j - 1] + gap;
            matrix[i][j] = diag.max(up).max(left);
        }
    }
    matrix[m][n]
}

pub fn edit_distance(seq1: &[u8], seq2: &[u8]) -> usize {
    let m = seq1.len();
    let n = seq2.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, val) in dp[0].iter_mut().enumerate() {
        *val = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if seq1[i - 1] == seq2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

pub fn hamming_distance(seq1: &[u8], seq2: &[u8]) -> usize {
    let n = seq1.len().min(seq2.len());
    (0..n).filter(|&i| seq1[i] != seq2[i]).count()
}

pub fn alignment_gc_content(seq: &[u8]) -> f64 {
    if seq.is_empty() {
        return 0.0;
    }
    let gc = seq
        .iter()
        .filter(|&&b| b == b'G' || b == b'C' || b == b'g' || b == b'c')
        .count();
    gc as f64 / seq.len() as f64
}

pub fn sequence_identity(seq1: &[u8], seq2: &[u8]) -> f64 {
    let n = seq1.len().min(seq2.len());
    if n == 0 {
        return 0.0;
    }
    let matches = (0..n).filter(|&i| seq1[i] == seq2[i]).count();
    matches as f64 / n as f64
}

pub fn codon_frequency(seq: &[u8]) -> Vec<(u32, usize)> {
    let mut counts = Vec::new();
    let mut i = 0;
    while i + 2 < seq.len() {
        let codon = (seq[i] as u32) << 16 | (seq[i + 1] as u32) << 8 | seq[i + 2] as u32;
        let mut found = false;
        for entry in counts.iter_mut() {
            let (c, count): &mut (u32, usize) = entry;
            if *c == codon {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            counts.push((codon, 1));
        }
        i += 3;
    }
    counts
}

pub fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    seq.iter()
        .rev()
        .map(|&b| match b {
            b'A' | b'a' => b'T',
            b'T' | b't' => b'A',
            b'G' | b'g' => b'C',
            b'C' | b'c' => b'G',
            other => other,
        })
        .collect()
}

pub fn melting_temperature_basic(
    a_count: usize,
    t_count: usize,
    g_count: usize,
    c_count: usize,
) -> f64 {
    let len = a_count + t_count + g_count + c_count;
    if len < 14 {
        2.0 * (a_count + t_count) as f64 + 4.0 * (g_count + c_count) as f64
    } else {
        64.9 + 41.0 * (g_count + c_count) as f64 / len as f64 - 500.0 / len as f64
    }
}
