use std::f64::consts::PI;

pub fn dct_ii(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|i| input[i] * (PI * (2 * i + 1) as f64 * k as f64 / (2 * n) as f64).cos())
                .sum()
        })
        .collect()
}

pub fn idct_ii(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    let scale = 2.0 / n as f64;
    (0..n)
        .map(|i| {
            let sum: f64 = input[0] * 0.5
                + (1..n)
                    .map(|k| input[k] * (PI * (2 * i + 1) as f64 * k as f64 / (2 * n) as f64).cos())
                    .sum::<f64>();
            sum * scale
        })
        .collect()
}

pub fn dst_i(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|i| input[i] * (PI * (i + 1) as f64 * (k + 1) as f64 / (n + 1) as f64).sin())
                .sum()
        })
        .collect()
}

pub fn dct_i(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    let nm1 = (n - 1) as f64;
    (0..n)
        .map(|k| {
            let mut sum = 0.0;
            for (i, &inp) in input.iter().enumerate() {
                let cos_val = (PI * i as f64 * k as f64 / nm1).cos();
                let weight = if i == 0 || i == n - 1 { 0.5 } else { 1.0 };
                sum += weight * inp * cos_val;
            }
            sum
        })
        .collect()
}

pub fn dct_iii(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|i| {
            let mut sum = input[0] * 0.5;
            for (k, &inp) in input.iter().enumerate().skip(1) {
                sum += inp * (PI * k as f64 * (2 * i + 1) as f64 / (2 * n) as f64).cos();
            }
            sum
        })
        .collect()
}

pub fn dct_iv(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|i| {
                    input[i] * (PI * (2 * i + 1) as f64 * (2 * k + 1) as f64 / (4 * n) as f64).cos()
                })
                .sum()
        })
        .collect()
}

pub fn dst_ii(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|i| {
                    input[i] * (PI * (2 * i + 1) as f64 * (k + 1) as f64 / (2 * n) as f64).sin()
                })
                .sum()
        })
        .collect()
}

pub fn dst_iii(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|i| {
            let mut sum = if n > 0 {
                input[n - 1] * if (i % 2) == 0 { 0.5 } else { -0.5 }
            } else {
                0.0
            };
            for (k, &inp) in input.iter().enumerate().take(n - 1) {
                sum += inp * (PI * (k + 1) as f64 * (2 * i + 1) as f64 / (2 * n) as f64).sin();
            }
            sum
        })
        .collect()
}

pub fn dst_iv(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|i| {
                    input[i] * (PI * (2 * i + 1) as f64 * (2 * k + 1) as f64 / (4 * n) as f64).sin()
                })
                .sum()
        })
        .collect()
}

pub fn mdct(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    let half = n / 2;
    (0..half)
        .map(|k| {
            (0..n)
                .map(|i| {
                    input[i]
                        * (PI / half as f64
                            * (i as f64 + 0.5 + half as f64 / 2.0)
                            * (k as f64 + 0.5))
                            .cos()
                })
                .sum()
        })
        .collect()
}

pub fn imdct(input: &[f64]) -> Vec<f64> {
    let half = input.len();
    let n = 2 * half;
    let scale = 2.0 / half as f64;
    (0..n)
        .map(|i| {
            let sum: f64 = (0..half)
                .map(|k| {
                    input[k]
                        * (PI / half as f64
                            * (i as f64 + 0.5 + half as f64 / 2.0)
                            * (k as f64 + 0.5))
                            .cos()
                })
                .sum();
            sum * scale
        })
        .collect()
}

pub fn dct_2d(input: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows: Vec<Vec<f64>> = input.iter().map(|row| dct_ii(row)).collect();
    let m = rows.len();
    let n = if m > 0 { rows[0].len() } else { 0 };
    let mut result = vec![vec![0.0; n]; m];
    for j in 0..n {
        let col: Vec<f64> = (0..m).map(|i| rows[i][j]).collect();
        let dct_col = dct_ii(&col);
        for i in 0..m {
            result[i][j] = dct_col[i];
        }
    }
    result
}

pub fn idct_2d(input: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let m = input.len();
    let n = if m > 0 { input[0].len() } else { 0 };
    let mut temp = vec![vec![0.0; n]; m];
    for j in 0..n {
        let col: Vec<f64> = (0..m).map(|i| input[i][j]).collect();
        let inv_col = idct_ii(&col);
        for i in 0..m {
            temp[i][j] = inv_col[i];
        }
    }
    temp.iter().map(|row| idct_ii(row)).collect()
}

pub fn hartley_transform(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|i| {
                    let angle = 2.0 * PI * i as f64 * k as f64 / n as f64;
                    input[i] * (angle.cos() + angle.sin())
                })
                .sum()
        })
        .collect()
}
