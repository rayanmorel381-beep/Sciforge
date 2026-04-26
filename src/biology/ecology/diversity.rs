pub fn shannon_diversity(abundances: &[f64]) -> f64 {
    let total: f64 = abundances.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let mut h = 0.0;
    for &n in abundances {
        if n > 0.0 {
            let p = n / total;
            h -= p * p.ln();
        }
    }
    h
}

pub fn simpson_diversity(abundances: &[f64]) -> f64 {
    let total: f64 = abundances.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let d: f64 = abundances
        .iter()
        .map(|&n| {
            let p = n / total;
            p * p
        })
        .sum();
    1.0 - d
}

pub fn inverse_simpson(abundances: &[f64]) -> f64 {
    let total: f64 = abundances.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let d: f64 = abundances
        .iter()
        .map(|&n| {
            let p = n / total;
            p * p
        })
        .sum();
    if d < 1e-30 {
        return 0.0;
    }
    1.0 / d
}

pub fn species_richness(abundances: &[f64]) -> usize {
    abundances.iter().filter(|&&n| n > 0.0).count()
}

pub fn pielou_evenness(abundances: &[f64]) -> f64 {
    let s = species_richness(abundances);
    if s <= 1 {
        return 1.0;
    }
    let h = shannon_diversity(abundances);
    h / (s as f64).ln()
}

pub fn berger_parker(abundances: &[f64]) -> f64 {
    let total: f64 = abundances.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let max_n = abundances.iter().cloned().fold(0.0_f64, f64::max);
    max_n / total
}

pub fn margalef_richness(species: usize, total_individuals: f64) -> f64 {
    if total_individuals <= 1.0 {
        return 0.0;
    }
    (species as f64 - 1.0) / total_individuals.ln()
}

pub fn chao1(observed: usize, singletons: usize, doubletons: usize) -> f64 {
    if doubletons == 0 {
        return observed as f64 + singletons as f64 * (singletons as f64 - 1.0) / 2.0;
    }
    observed as f64 + (singletons as f64 * singletons as f64) / (2.0 * doubletons as f64)
}

pub fn hill_number(abundances: &[f64], q: f64) -> f64 {
    let total: f64 = abundances.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    if (q - 1.0).abs() < 1e-12 {
        return shannon_diversity(abundances).exp();
    }
    let sum: f64 = abundances
        .iter()
        .filter(|&&n| n > 0.0)
        .map(|&n| (n / total).powf(q))
        .sum();
    sum.powf(1.0 / (1.0 - q))
}
