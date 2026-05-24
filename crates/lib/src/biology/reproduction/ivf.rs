pub fn ivf_cycle_success_rate(age: f64, embryo_quality: f64, endometrial_thickness: f64) -> f64 {
    let age_factor = if age < 35.0 {
        1.0
    } else {
        (1.0 - 0.03 * (age - 35.0)).max(0.1)
    };
    let endo_factor = if (7.0..=14.0).contains(&endometrial_thickness) {
        1.0
    } else {
        0.5
    };
    (embryo_quality * age_factor * endo_factor * 0.5).min(1.0)
}

pub fn ovarian_reserve_amh(amh_ng_ml: f64) -> &'static str {
    if amh_ng_ml > 3.5 {
        "high"
    } else if amh_ng_ml > 1.0 {
        "normal"
    } else if amh_ng_ml > 0.5 {
        "low"
    } else {
        "very low"
    }
}

pub fn antral_follicle_response(fsh_dose: f64, sensitivity: f64, max_follicles: f64) -> f64 {
    max_follicles * (1.0 - (-sensitivity * fsh_dose).exp())
}

pub fn ohss_risk(estradiol: f64, follicle_count: usize, bmi: f64) -> f64 {
    let e2_factor = estradiol / 3000.0;
    let follicle_factor = follicle_count as f64 / 20.0;
    let bmi_factor = if bmi < 25.0 { 1.2 } else { 0.8 };
    (e2_factor * follicle_factor * bmi_factor).min(1.0)
}

pub fn embryo_grading_score(cell_count: usize, fragmentation_pct: f64, symmetry: f64) -> f64 {
    let cell_score = if cell_count >= 8 {
        1.0
    } else {
        cell_count as f64 / 8.0
    };
    let frag_score = 1.0 - fragmentation_pct / 100.0;
    cell_score * frag_score * symmetry
}

pub fn blastocyst_expansion_rate(hours_post_fertilization: f64) -> f64 {
    if hours_post_fertilization < 96.0 {
        0.0
    } else {
        ((hours_post_fertilization - 96.0) / 24.0).min(1.0)
    }
}

pub fn cryopreservation_survival(cooling_rate: f64, optimal_rate: f64, cpa_conc: f64) -> f64 {
    let rate_factor =
        (-(cooling_rate - optimal_rate).powi(2) / (optimal_rate * optimal_rate)).exp();
    let cpa_factor = cpa_conc / (cpa_conc + 1.0);
    rate_factor * cpa_factor
}

pub fn cumulative_ivf_pregnancy_rate(cycle_rate: f64, cycles: usize) -> f64 {
    1.0 - (1.0 - cycle_rate).powi(cycles as i32)
}

pub fn sperm_dna_fragmentation_impact(dfi: f64, baseline_fertility: f64) -> f64 {
    if dfi < 15.0 {
        baseline_fertility
    } else if dfi < 30.0 {
        baseline_fertility * (1.0 - (dfi - 15.0) / 30.0)
    } else {
        baseline_fertility * 0.5 * (-0.02 * (dfi - 30.0)).exp()
    }
}
