pub fn b_ion_masses(sequence: &str) -> Vec<f64> {
    let mut masses = Vec::new();
    let mut cumulative = 0.0;
    let chars: Vec<char> = sequence.chars().collect();
    for &ch in chars.iter().take(chars.len().saturating_sub(1)) {
        cumulative += residue_mass(ch);
        masses.push(cumulative + 1.00728);
    }
    masses
}

pub fn y_ion_masses(sequence: &str) -> Vec<f64> {
    let mut masses = Vec::new();
    let chars: Vec<char> = sequence.chars().collect();
    let mut cumulative = 18.01056;
    for i in (1..chars.len()).rev() {
        cumulative += residue_mass(chars[i]);
        masses.push(cumulative + 1.00728);
    }
    masses.reverse();
    masses
}

fn residue_mass(aa: char) -> f64 {
    match aa {
        'G' => 57.02146,
        'A' => 71.03711,
        'V' => 99.06841,
        'L' => 113.08406,
        'I' => 113.08406,
        'P' => 97.05276,
        'F' => 147.06841,
        'W' => 186.07931,
        'M' => 131.04049,
        'S' => 87.03203,
        'T' => 101.04768,
        'C' => 103.00919,
        'Y' => 163.06333,
        'H' => 137.05891,
        'D' => 115.02694,
        'E' => 129.04259,
        'N' => 114.04293,
        'Q' => 128.05858,
        'K' => 128.09496,
        'R' => 156.10111,
        _ => 0.0,
    }
}

pub fn mz_ratio(mass: f64, charge: usize) -> f64 {
    (mass + charge as f64 * 1.00728) / charge as f64
}

pub fn mass_from_mz(mz: f64, charge: usize) -> f64 {
    mz * charge as f64 - charge as f64 * 1.00728
}

pub fn mass_accuracy_ppm(theoretical: f64, observed: f64) -> f64 {
    ((observed - theoretical) / theoretical) * 1e6
}

pub fn isotope_pattern_averagine(mass: f64, n_peaks: usize) -> Vec<f64> {
    let avg_mass = 111.1254;
    let n_amino = (mass / avg_mass).round();
    let n_c = (n_amino * 4.9384).round();
    let lambda = n_c * 0.0111;
    let mut pattern = Vec::with_capacity(n_peaks);
    let mut poisson = (-lambda).exp();
    pattern.push(poisson);
    for k in 1..n_peaks {
        poisson *= lambda / k as f64;
        pattern.push(poisson);
    }
    let total: f64 = pattern.iter().sum();
    for p in &mut pattern {
        *p /= total.max(1e-30);
    }
    pattern
}
