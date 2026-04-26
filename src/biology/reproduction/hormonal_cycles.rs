pub fn menstrual_cycle_hormone(day: f64, hormone: &str) -> f64 {
    let pi2 = 2.0 * std::f64::consts::PI;
    match hormone {
        "estrogen" => {
            50.0 + 150.0 * (-(day - 13.0).powi(2) / 8.0).exp()
                + 80.0 * (-(day - 21.0).powi(2) / 12.0).exp()
        }
        "progesterone" => {
            if day < 14.0 {
                1.0
            } else {
                1.0 + 15.0 * (-(day - 21.0).powi(2) / 18.0).exp()
            }
        }
        "lh" => 5.0 + 60.0 * (-(day - 14.0).powi(2) / 2.0).exp(),
        "fsh" => {
            8.0 + 10.0 * (-(day - 3.0).powi(2) / 4.0).exp()
                + 5.0 * (-(day - 14.0).powi(2) / 2.0).exp()
        }
        _ => 50.0 * (pi2 * day / 28.0).sin().abs(),
    }
}

pub fn ovulation_probability(lh_surge: f64, follicle_maturity: f64, threshold: f64) -> f64 {
    if lh_surge < threshold {
        return 0.0;
    }
    follicle_maturity * (lh_surge - threshold) / lh_surge
}

pub fn endometrial_thickness(day: f64, estrogen: f64) -> f64 {
    let base = 2.0;
    let growth = estrogen / 200.0 * 10.0;
    if day < 5.0 {
        base
    } else if day < 14.0 {
        base + growth * (day - 5.0) / 9.0
    } else if day < 28.0 {
        base + growth
    } else {
        base
    }
}

pub fn fertility_window(cycle_day: f64, cycle_length: f64) -> f64 {
    let ovulation_day = cycle_length - 14.0;
    let distance = (cycle_day - ovulation_day).abs();
    if distance > 6.0 {
        0.0
    } else {
        (1.0 - distance / 6.0).max(0.0)
    }
}

pub fn hcg_doubling_time(initial_hcg: f64, days: f64, doubling_time: f64) -> f64 {
    initial_hcg * 2.0_f64.powf(days / doubling_time)
}

pub fn implantation_probability(
    embryo_quality: f64,
    endometrial_receptivity: f64,
    age_factor: f64,
) -> f64 {
    (embryo_quality * endometrial_receptivity * age_factor).min(1.0)
}

pub fn spermatogenesis_duration_days() -> f64 {
    74.0
}

pub fn sperm_motility_score(progressive: f64, non_progressive: f64, immotile: f64) -> f64 {
    progressive / (progressive + non_progressive + immotile).max(1e-30)
}

pub fn testosterone_circadian(hour: f64, peak_level: f64, trough_level: f64) -> f64 {
    let amplitude = (peak_level - trough_level) / 2.0;
    let mesor = (peak_level + trough_level) / 2.0;
    mesor + amplitude * (2.0 * std::f64::consts::PI * (hour - 8.0) / 24.0).cos()
}
