pub fn ovarian_cycle_hormone(
    t: f64,
    amplitude: f64,
    peak_day: f64,
    width: f64,
    baseline: f64,
) -> f64 {
    baseline + amplitude * (-0.5 * ((t - peak_day) / width).powi(2)).exp()
}

pub fn follicle_growth(diameter: f64, fsh: f64, growth_rate: f64, max_diameter: f64) -> f64 {
    growth_rate * fsh * diameter * (1.0 - diameter / max_diameter)
}

pub fn sperm_motility_fraction(velocity: f64, threshold: f64, concentration: f64) -> f64 {
    let motile_fraction = 1.0 / (1.0 + (-10.0 * (velocity - threshold)).exp());
    concentration * motile_fraction
}

pub fn sperm_capacitation_rate(t: f64, half_time: f64) -> f64 {
    1.0 - (-std::f64::consts::LN_2 * t / half_time).exp()
}

pub fn fertilization_probability(sperm_count: f64, half_max: f64) -> f64 {
    sperm_count / (sperm_count + half_max)
}

pub fn implantation_window(progesterone: f64, threshold: f64, estrogen_ratio: f64) -> bool {
    progesterone > threshold && estrogen_ratio > 0.5 && estrogen_ratio < 3.0
}

pub fn hcg_doubling(initial: f64, doubling_time: f64, t: f64) -> f64 {
    initial * (2.0_f64).powf(t / doubling_time)
}

pub fn lh_surge_model(t: f64, t_peak: f64, amplitude: f64, rise_rate: f64, fall_rate: f64) -> f64 {
    if t < t_peak {
        amplitude * (rise_rate * (t - t_peak)).exp()
    } else {
        amplitude * (-fall_rate * (t - t_peak)).exp()
    }
}

pub fn estradiol_follicular(follicle_diameter: f64, num_follicles: f64, sensitivity: f64) -> f64 {
    sensitivity * num_follicles * follicle_diameter.powi(2)
}

pub fn progesterone_luteal(
    t_post_ovulation: f64,
    peak: f64,
    rise_rate: f64,
    fall_rate: f64,
) -> f64 {
    let t_peak = 7.0;
    if t_post_ovulation < t_peak {
        peak * (1.0 - (-rise_rate * t_post_ovulation).exp())
    } else {
        peak * (-fall_rate * (t_post_ovulation - t_peak)).exp()
    }
}

pub fn oocyte_quality_age(
    base_quality: f64,
    age: f64,
    decline_start: f64,
    decline_rate: f64,
) -> f64 {
    if age < decline_start {
        base_quality
    } else {
        base_quality * (-decline_rate * (age - decline_start)).exp()
    }
}

pub fn antral_follicle_count(age: f64, initial_pool: f64, depletion_rate: f64) -> f64 {
    initial_pool * (-depletion_rate * age).exp()
}

pub fn anti_mullerian_hormone(follicle_count: f64, sensitivity: f64) -> f64 {
    sensitivity * follicle_count
}

pub fn ivf_success_rate(age: f64, embryo_quality: f64, endometrial_thickness: f64) -> f64 {
    let age_factor = if age < 35.0 {
        1.0
    } else {
        (-0.05 * (age - 35.0)).exp()
    };
    let endo_factor = 1.0 / (1.0 + (-2.0 * (endometrial_thickness - 7.0)).exp());
    embryo_quality * age_factor * endo_factor
}

pub fn menstrual_cycle_length(lh_peak_day: f64, luteal_phase_length: f64) -> f64 {
    lh_peak_day + luteal_phase_length
}

pub fn sperm_concentration_fertility(concentration: f64, motility: f64, morphology: f64) -> f64 {
    concentration * motility * morphology
}

pub fn cumulative_pregnancy_rate(monthly_fecundability: f64, months: u32) -> f64 {
    1.0 - (1.0 - monthly_fecundability).powi(months as i32)
}

pub fn zona_pellucida_binding(receptors: f64, sperm_conc: f64, kd: f64) -> f64 {
    receptors * sperm_conc / (kd + sperm_conc)
}

pub fn acrosome_reaction_rate(capacitated_fraction: f64, zona_signal: f64, k: f64) -> f64 {
    k * capacitated_fraction * zona_signal
}

pub fn endometrial_receptivity(p4: f64, lif: f64, integrin: f64, threshold_p4: f64) -> f64 {
    let p4_factor = p4 / (p4 + threshold_p4);
    p4_factor * lif * integrin
}

pub fn twin_probability_dizygotic(age: f64, fsh_level: f64) -> f64 {
    let age_factor = 1.0 / (1.0 + (-0.3 * (age - 35.0)).exp());
    0.01 * (1.0 + age_factor + 0.1 * fsh_level)
}
