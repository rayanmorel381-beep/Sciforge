pub fn peptide_molecular_weight(sequence: &str) -> f64 {
    let water = 18.01528;
    let mut mw = water;
    for aa in sequence.chars() {
        mw += amino_acid_mass(aa);
    }
    mw
}

fn amino_acid_mass(aa: char) -> f64 {
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

pub fn isoelectric_point(sequence: &str) -> f64 {
    let mut ph_low = 0.0_f64;
    let mut ph_high = 14.0_f64;
    for _ in 0..100 {
        let ph = (ph_low + ph_high) / 2.0;
        let charge = net_charge_at_ph(sequence, ph);
        if charge > 0.0 {
            ph_low = ph;
        } else {
            ph_high = ph;
        }
    }
    (ph_low + ph_high) / 2.0
}

fn net_charge_at_ph(sequence: &str, ph: f64) -> f64 {
    let mut charge = henderson_hasselbalch(ph, 8.0, true);
    charge += henderson_hasselbalch(ph, 3.1, false);
    for aa in sequence.chars() {
        match aa {
            'K' => charge += henderson_hasselbalch(ph, 10.5, true),
            'R' => charge += henderson_hasselbalch(ph, 12.5, true),
            'H' => charge += henderson_hasselbalch(ph, 6.0, true),
            'D' => charge += henderson_hasselbalch(ph, 3.9, false),
            'E' => charge += henderson_hasselbalch(ph, 4.1, false),
            'C' => charge += henderson_hasselbalch(ph, 8.3, false),
            'Y' => charge += henderson_hasselbalch(ph, 10.1, false),
            _ => {}
        }
    }
    charge
}

fn henderson_hasselbalch(ph: f64, pka: f64, is_positive: bool) -> f64 {
    if is_positive {
        1.0 / (1.0 + (10.0_f64).powf(ph - pka))
    } else {
        -1.0 / (1.0 + (10.0_f64).powf(pka - ph))
    }
}

pub fn gravy_index(sequence: &str) -> f64 {
    let n = sequence.len();
    if n == 0 {
        return 0.0;
    }
    let total: f64 = sequence.chars().map(hydropathy).sum();
    total / n as f64
}

fn hydropathy(aa: char) -> f64 {
    match aa {
        'I' => 4.5,
        'V' => 4.2,
        'L' => 3.8,
        'F' => 2.8,
        'C' => 2.5,
        'M' => 1.9,
        'A' => 1.8,
        'G' => -0.4,
        'T' => -0.7,
        'S' => -0.8,
        'W' => -0.9,
        'Y' => -1.3,
        'P' => -1.6,
        'H' => -3.2,
        'D' => -3.5,
        'E' => -3.5,
        'N' => -3.5,
        'Q' => -3.5,
        'K' => -3.9,
        'R' => -4.5,
        _ => 0.0,
    }
}

pub fn extinction_coefficient_280(n_trp: usize, n_tyr: usize, n_cystine: usize) -> f64 {
    n_trp as f64 * 5500.0 + n_tyr as f64 * 1490.0 + n_cystine as f64 * 125.0
}
