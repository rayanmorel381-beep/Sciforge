pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).filter(|&(&x, &y)| x != y).count()
}

pub fn p_distance(a: &[u8], b: &[u8]) -> f64 {
    let d = hamming_distance(a, b);
    d as f64 / a.len() as f64
}

pub fn jukes_cantor(p: f64) -> f64 {
    if p >= 0.75 {
        return f64::INFINITY;
    }
    -0.75 * (1.0 - 4.0 * p / 3.0).ln()
}

pub fn kimura_2p(transitions: f64, transversions: f64, length: f64) -> f64 {
    let p = transitions / length;
    let q = transversions / length;
    let t1 = 1.0 - 2.0 * p - q;
    let t2 = 1.0 - 2.0 * q;
    if t1 <= 0.0 || t2 <= 0.0 {
        return f64::INFINITY;
    }
    -0.5 * t1.ln() - 0.25 * t2.ln()
}

pub fn count_transitions_transversions(a: &[u8], b: &[u8]) -> (usize, usize) {
    let mut transitions = 0;
    let mut transversions = 0;
    let purines = [b'A', b'G'];
    let pyrimidines = [b'C', b'T'];
    for (&x, &y) in a.iter().zip(b.iter()) {
        if x == y {
            continue;
        }
        let x_purine = purines.contains(&x);
        let y_purine = purines.contains(&y);
        let x_pyrimidine = pyrimidines.contains(&x);
        let y_pyrimidine = pyrimidines.contains(&y);
        if (x_purine && y_purine) || (x_pyrimidine && y_pyrimidine) {
            transitions += 1;
        } else {
            transversions += 1;
        }
    }
    (transitions, transversions)
}

pub fn distance_matrix(sequences: &[&[u8]]) -> Vec<Vec<f64>> {
    let n = sequences.len();
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let p = p_distance(sequences[i], sequences[j]);
            let d = jukes_cantor(p);
            matrix[i][j] = d;
            matrix[j][i] = d;
        }
    }
    matrix
}

pub fn log_det_distance(freq_matrix: &[[f64; 4]; 4]) -> f64 {
    let det = determinant_4x4(freq_matrix);
    if det <= 0.0 {
        return f64::INFINITY;
    }
    -det.ln() / 4.0
}

fn determinant_4x4(m: &[[f64; 4]; 4]) -> f64 {
    let mut result = 0.0;
    for j in 0..4 {
        let mut sub = [[0.0; 3]; 3];
        for i in 1..4 {
            let mut col = 0;
            for (k, &val) in m[i].iter().enumerate() {
                if k == j {
                    continue;
                }
                sub[i - 1][col] = val;
                col += 1;
            }
        }
        let minor = sub[0][0] * (sub[1][1] * sub[2][2] - sub[1][2] * sub[2][1])
            - sub[0][1] * (sub[1][0] * sub[2][2] - sub[1][2] * sub[2][0])
            + sub[0][2] * (sub[1][0] * sub[2][1] - sub[1][1] * sub[2][0]);
        let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
        result += sign * m[0][j] * minor;
    }
    result
}
