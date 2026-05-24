pub const BLOSUM62: [[i8; 20]; 20] = [
    [
        4, -1, -2, -2, 0, -1, -1, 0, -2, -1, -1, -1, -1, -2, -1, 1, 0, -3, -2, 0,
    ],
    [
        -1, 5, 0, -2, -3, 1, 0, -2, 0, -3, -2, 2, -1, -3, -2, -1, -1, -3, -2, -3,
    ],
    [
        -2, 0, 6, 1, -3, 0, 0, 0, 1, -3, -3, 0, -2, -3, -2, 1, 0, -4, -2, -3,
    ],
    [
        -2, -2, 1, 6, -3, 0, 2, -1, -1, -3, -4, -1, -3, -3, -1, 0, -1, -4, -3, -3,
    ],
    [
        0, -3, -3, -3, 9, -3, -4, -3, -3, -1, -1, -3, -1, -2, -3, -1, -1, -2, -2, -1,
    ],
    [
        -1, 1, 0, 0, -3, 5, 2, -2, 0, -3, -2, 1, 0, -3, -1, 0, -1, -2, -1, -2,
    ],
    [
        -1, 0, 0, 2, -4, 2, 5, -2, 0, -3, -3, 1, -2, -3, -1, 0, -1, -3, -2, -2,
    ],
    [
        0, -2, 0, -1, -3, -2, -2, 6, -2, -4, -4, -2, -3, -3, -2, 0, -2, -2, -3, -3,
    ],
    [
        -2, 0, 1, -1, -3, 0, 0, -2, 8, -3, -3, -1, -2, -1, -2, -1, -2, -2, 2, -3,
    ],
    [
        -1, -3, -3, -3, -1, -3, -3, -4, -3, 4, 2, -3, 1, 0, -3, -2, -1, -3, -1, 3,
    ],
    [
        -1, -2, -3, -4, -1, -2, -3, -4, -3, 2, 4, -2, 2, 0, -3, -2, -1, -2, -1, 1,
    ],
    [
        -1, 2, 0, -1, -3, 1, 1, -2, -1, -3, -2, 5, -1, -3, -1, 0, -1, -3, -2, -2,
    ],
    [
        -1, -1, -2, -3, -1, 0, -2, -3, -2, 1, 2, -1, 5, 0, -2, -1, -1, -1, -1, 1,
    ],
    [
        -2, -3, -3, -3, -2, -3, -3, -3, -1, 0, 0, -3, 0, 6, -4, -2, -2, 1, 3, -1,
    ],
    [
        -1, -2, -2, -1, -3, -1, -1, -2, -2, -3, -3, -1, -2, -4, 7, -1, -1, -4, -3, -2,
    ],
    [
        1, -1, 1, 0, -1, 0, 0, 0, -1, -2, -2, 0, -1, -2, -1, 4, 1, -3, -2, -2,
    ],
    [
        0, -1, 0, -1, -1, -1, -1, -2, -2, -1, -1, -1, -1, -2, -1, 1, 5, -2, -2, 0,
    ],
    [
        -3, -3, -4, -4, -2, -2, -3, -2, -2, -3, -2, -3, -1, 1, -4, -3, -2, 11, 2, -3,
    ],
    [
        -2, -2, -2, -3, -2, -1, -2, -3, 2, -1, -1, -2, -1, 3, -3, -2, -2, 2, 7, -1,
    ],
    [
        0, -3, -3, -3, -1, -2, -2, -3, -3, 3, 1, -2, 1, -1, -2, -2, 0, -3, -1, 4,
    ],
];

pub fn aa_to_index(aa: u8) -> Option<usize> {
    match aa {
        b'A' => Some(0),
        b'R' => Some(1),
        b'N' => Some(2),
        b'D' => Some(3),
        b'C' => Some(4),
        b'Q' => Some(5),
        b'E' => Some(6),
        b'G' => Some(7),
        b'H' => Some(8),
        b'I' => Some(9),
        b'L' => Some(10),
        b'K' => Some(11),
        b'M' => Some(12),
        b'F' => Some(13),
        b'P' => Some(14),
        b'S' => Some(15),
        b'T' => Some(16),
        b'W' => Some(17),
        b'Y' => Some(18),
        b'V' => Some(19),
        _ => None,
    }
}

pub fn blosum62_score(a: u8, b: u8) -> i8 {
    match (aa_to_index(a), aa_to_index(b)) {
        (Some(i), Some(j)) => BLOSUM62[i][j],
        _ => 0,
    }
}

pub fn needleman_wunsch(seq_a: &[u8], seq_b: &[u8], gap_penalty: i32) -> (Vec<u8>, Vec<u8>, i32) {
    let m = seq_a.len();
    let n = seq_b.len();
    let mut score = vec![vec![0i32; n + 1]; m + 1];

    for (i, si) in score.iter_mut().enumerate() {
        si[0] = gap_penalty * i as i32;
    }
    for (j, s0j) in score[0].iter_mut().enumerate() {
        *s0j = gap_penalty * j as i32;
    }

    for i in 1..=m {
        for j in 1..=n {
            let match_score =
                score[i - 1][j - 1] + blosum62_score(seq_a[i - 1], seq_b[j - 1]) as i32;
            let delete = score[i - 1][j] + gap_penalty;
            let insert = score[i][j - 1] + gap_penalty;
            score[i][j] = match_score.max(delete).max(insert);
        }
    }

    let mut align_a = Vec::new();
    let mut align_b = Vec::new();
    let mut i = m;
    let mut j = n;
    while i > 0 || j > 0 {
        if i > 0
            && j > 0
            && score[i][j]
                == score[i - 1][j - 1] + blosum62_score(seq_a[i - 1], seq_b[j - 1]) as i32
        {
            align_a.push(seq_a[i - 1]);
            align_b.push(seq_b[j - 1]);
            i -= 1;
            j -= 1;
        } else if i > 0 && score[i][j] == score[i - 1][j] + gap_penalty {
            align_a.push(seq_a[i - 1]);
            align_b.push(b'-');
            i -= 1;
        } else {
            align_a.push(b'-');
            align_b.push(seq_b[j - 1]);
            j -= 1;
        }
    }
    align_a.reverse();
    align_b.reverse();
    (align_a, align_b, score[m][n])
}

pub fn smith_waterman(seq_a: &[u8], seq_b: &[u8], gap_penalty: i32) -> (Vec<u8>, Vec<u8>, i32) {
    let m = seq_a.len();
    let n = seq_b.len();
    let mut score = vec![vec![0i32; n + 1]; m + 1];
    let mut max_score = 0;
    let mut max_i = 0;
    let mut max_j = 0;

    for i in 1..=m {
        for j in 1..=n {
            let match_s = score[i - 1][j - 1] + blosum62_score(seq_a[i - 1], seq_b[j - 1]) as i32;
            let delete = score[i - 1][j] + gap_penalty;
            let insert = score[i][j - 1] + gap_penalty;
            score[i][j] = 0_i32.max(match_s).max(delete).max(insert);
            if score[i][j] > max_score {
                max_score = score[i][j];
                max_i = i;
                max_j = j;
            }
        }
    }

    let mut align_a = Vec::new();
    let mut align_b = Vec::new();
    let mut i = max_i;
    let mut j = max_j;
    while i > 0 && j > 0 && score[i][j] > 0 {
        if score[i][j] == score[i - 1][j - 1] + blosum62_score(seq_a[i - 1], seq_b[j - 1]) as i32 {
            align_a.push(seq_a[i - 1]);
            align_b.push(seq_b[j - 1]);
            i -= 1;
            j -= 1;
        } else if score[i][j] == score[i - 1][j] + gap_penalty {
            align_a.push(seq_a[i - 1]);
            align_b.push(b'-');
            i -= 1;
        } else {
            align_a.push(b'-');
            align_b.push(seq_b[j - 1]);
            j -= 1;
        }
    }
    align_a.reverse();
    align_b.reverse();
    (align_a, align_b, max_score)
}

pub fn alignment_identity(align_a: &[u8], align_b: &[u8]) -> f64 {
    let matches = align_a
        .iter()
        .zip(align_b.iter())
        .filter(|&(&a, &b)| a != b'-' && b != b'-' && a == b)
        .count();
    let total = align_a.len();
    if total == 0 {
        return 0.0;
    }
    matches as f64 / total as f64
}

pub fn affine_gap_needleman_wunsch(
    seq_a: &[u8],
    seq_b: &[u8],
    gap_open: i32,
    gap_extend: i32,
) -> (Vec<u8>, Vec<u8>, i32) {
    let m = seq_a.len();
    let n = seq_b.len();
    let neg_inf = i32::MIN / 2;

    let mut m_mat = vec![vec![neg_inf; n + 1]; m + 1];
    let mut ix = vec![vec![neg_inf; n + 1]; m + 1];
    let mut iy = vec![vec![neg_inf; n + 1]; m + 1];

    m_mat[0][0] = 0;
    for (i, ixi) in ix.iter_mut().enumerate().skip(1) {
        ixi[0] = gap_open + (i as i32 - 1) * gap_extend;
    }
    for (j, iy0j) in iy[0].iter_mut().enumerate().skip(1) {
        *iy0j = gap_open + (j as i32 - 1) * gap_extend;
    }

    for i in 1..=m {
        for j in 1..=n {
            let s = blosum62_score(seq_a[i - 1], seq_b[j - 1]) as i32;
            m_mat[i][j] = s + m_mat[i - 1][j - 1]
                .max(ix[i - 1][j - 1])
                .max(iy[i - 1][j - 1]);
            ix[i][j] = (m_mat[i - 1][j] + gap_open).max(ix[i - 1][j] + gap_extend);
            iy[i][j] = (m_mat[i][j - 1] + gap_open).max(iy[i][j - 1] + gap_extend);
        }
    }

    let final_score = m_mat[m][n].max(ix[m][n]).max(iy[m][n]);
    let mut align_a = Vec::new();
    let mut align_b = Vec::new();
    let mut i = m;
    let mut j = n;
    let mut state = if m_mat[m][n] >= ix[m][n] && m_mat[m][n] >= iy[m][n] {
        0
    } else if ix[m][n] >= iy[m][n] {
        1
    } else {
        2
    };

    while i > 0 || j > 0 {
        match state {
            0 => {
                if i == 0 || j == 0 {
                    break;
                }
                align_a.push(seq_a[i - 1]);
                align_b.push(seq_b[j - 1]);
                let prev = m_mat[i - 1][j - 1]
                    .max(ix[i - 1][j - 1])
                    .max(iy[i - 1][j - 1]);
                if i > 0 && j > 0 && prev == ix[i - 1][j - 1] {
                    state = 1;
                } else if i > 0 && j > 0 && prev == iy[i - 1][j - 1] {
                    state = 2;
                }
                i -= 1;
                j -= 1;
            }
            1 => {
                if i == 0 {
                    break;
                }
                align_a.push(seq_a[i - 1]);
                align_b.push(b'-');
                if ix[i][j] == ix[i - 1][j] + gap_extend {
                    state = 1;
                } else {
                    state = 0;
                }
                i -= 1;
            }
            _ => {
                if j == 0 {
                    break;
                }
                align_a.push(b'-');
                align_b.push(seq_b[j - 1]);
                if iy[i][j] == iy[i][j - 1] + gap_extend {
                    state = 2;
                } else {
                    state = 0;
                }
                j -= 1;
            }
        }
    }
    align_a.reverse();
    align_b.reverse();
    (align_a, align_b, final_score)
}

pub fn multiple_alignment_score(alignment: &[Vec<u8>]) -> i32 {
    if alignment.len() < 2 {
        return 0;
    }
    let cols = alignment.iter().map(|s| s.len()).min().unwrap_or(0);
    let mut total = 0i32;
    for col in 0..cols {
        for i in 0..alignment.len() {
            for j in (i + 1)..alignment.len() {
                let a = alignment[i][col];
                let b = alignment[j][col];
                if a != b'-' && b != b'-' {
                    total += blosum62_score(a, b) as i32;
                }
            }
        }
    }
    total
}

pub fn pairwise_distance(align_a: &[u8], align_b: &[u8]) -> f64 {
    let mut mismatches = 0;
    let mut compared = 0;
    for (&a, &b) in align_a.iter().zip(align_b.iter()) {
        if a != b'-' && b != b'-' {
            compared += 1;
            if a != b {
                mismatches += 1;
            }
        }
    }
    if compared == 0 {
        return 1.0;
    }
    mismatches as f64 / compared as f64
}

pub fn jukes_cantor_distance(p_distance: f64) -> f64 {
    if p_distance >= 0.75 {
        return f64::INFINITY;
    }
    -0.75 * (1.0 - 4.0 * p_distance / 3.0).ln()
}

pub fn gap_fraction(aligned: &[u8]) -> f64 {
    if aligned.is_empty() {
        return 0.0;
    }
    let gaps = aligned.iter().filter(|&&b| b == b'-').count();
    gaps as f64 / aligned.len() as f64
}
