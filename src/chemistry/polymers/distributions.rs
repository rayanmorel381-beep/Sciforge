pub fn number_average_molar_mass(ni: &[f64], mi: &[f64]) -> f64 {
    let total_n: f64 = ni.iter().sum();
    if total_n < 1e-30 {
        return 0.0;
    }
    ni.iter().zip(mi.iter()).map(|(n, m)| n * m).sum::<f64>() / total_n
}

pub fn weight_average_molar_mass(ni: &[f64], mi: &[f64]) -> f64 {
    let total_nm: f64 = ni.iter().zip(mi.iter()).map(|(n, m)| n * m).sum();
    if total_nm < 1e-30 {
        return 0.0;
    }
    ni.iter()
        .zip(mi.iter())
        .map(|(n, m)| n * m * m)
        .sum::<f64>()
        / total_nm
}

pub fn z_average_molar_mass(ni: &[f64], mi: &[f64]) -> f64 {
    let total_nm2: f64 = ni.iter().zip(mi.iter()).map(|(n, m)| n * m * m).sum();
    if total_nm2 < 1e-30 {
        return 0.0;
    }
    ni.iter()
        .zip(mi.iter())
        .map(|(n, m)| n * m * m * m)
        .sum::<f64>()
        / total_nm2
}

pub fn schulz_flory_distribution(p: f64, x: u32) -> f64 {
    (1.0 - p) * p.powi(x as i32 - 1)
}

pub fn most_probable_chain_length(p: f64) -> f64 {
    -1.0 / p.max(1e-30).ln()
}

pub fn flory_huggins_free_energy(phi: f64, n1: f64, n2: f64, chi: f64, temperature: f64) -> f64 {
    crate::constants::R_GAS
        * temperature
        * (phi / n1.max(1e-30) * phi.max(1e-30).ln()
            + (1.0 - phi) / n2.max(1e-30) * (1.0 - phi).max(1e-30).ln()
            + chi * phi * (1.0 - phi))
}
