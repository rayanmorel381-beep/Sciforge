pub fn oxidative_stress_index(ros_production: f64, antioxidant_capacity: f64) -> f64 {
    if antioxidant_capacity.abs() < 1e-15 {
        return f64::INFINITY;
    }
    ros_production / antioxidant_capacity
}

pub fn dna_adduct_formation_rate(
    reactive_metabolite_conc: f64,
    dna_conc: f64,
    k_adduct: f64,
) -> f64 {
    k_adduct * reactive_metabolite_conc * dna_conc
}

pub fn dose_response_hill(dose: f64, emax: f64, ec50: f64, n: f64) -> f64 {
    emax * dose.powf(n) / (ec50.powf(n) + dose.powf(n))
}

pub fn bmd_estimate(background: f64, bmr: f64, slope: f64) -> f64 {
    if slope.abs() < 1e-15 {
        return 0.0;
    }
    ((background + bmr) / background).ln() / slope
}

pub fn hepatotoxicity_clearance_ratio(metabolite_formation: f64, metabolite_detox: f64) -> f64 {
    if metabolite_detox.abs() < 1e-15 {
        return f64::INFINITY;
    }
    metabolite_formation / metabolite_detox
}

pub fn receptor_mediated_toxicity(ligand: f64, receptor_total: f64, kd: f64) -> f64 {
    receptor_total * ligand / (kd + ligand)
}

pub fn safety_margin(noael: f64, human_exposure: f64) -> f64 {
    if human_exposure.abs() < 1e-15 {
        return f64::INFINITY;
    }
    noael / human_exposure
}

pub fn allometric_dose_scaling(animal_dose: f64, animal_weight: f64, human_weight: f64) -> f64 {
    animal_dose * (human_weight / animal_weight).powf(0.75)
}

pub fn cytotoxicity_viability(live_cells: f64, total_cells: f64) -> f64 {
    if total_cells.abs() < 1e-15 {
        return 0.0;
    }
    live_cells / total_cells
}

pub fn genotoxicity_micronucleus_rate(micronuclei: f64, cells_scored: f64) -> f64 {
    if cells_scored.abs() < 1e-15 {
        return 0.0;
    }
    micronuclei / cells_scored * 1000.0
}

pub fn ames_mutagenicity_ratio(revertants_treated: f64, revertants_control: f64) -> f64 {
    if revertants_control.abs() < 1e-15 {
        return 0.0;
    }
    revertants_treated / revertants_control
}
