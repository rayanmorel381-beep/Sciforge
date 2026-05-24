pub fn autophagy_flux(lc3_ii: f64, p62: f64, bafilomycin_effect: f64) -> f64 {
    lc3_ii * (1.0 + bafilomycin_effect) / (p62 + 1.0)
}

pub fn proteasome_degradation_rate(ubiquitin_tags: f64, proteasome_activity: f64, km: f64) -> f64 {
    proteasome_activity * ubiquitin_tags / (km + ubiquitin_tags)
}

pub fn lysosome_ph(v_atpase_rate: f64, proton_leak: f64, buffer_capacity: f64, volume: f64) -> f64 {
    let net_h = (v_atpase_rate - proton_leak) / (buffer_capacity * volume).max(1e-30);
    -net_h.log10().clamp(0.0, 14.0)
}

pub fn endosome_maturation(rab5: f64, rab7: f64, conversion_rate: f64, dt: f64) -> (f64, f64) {
    let converted = conversion_rate * rab5 * dt;
    ((rab5 - converted).max(0.0), rab7 + converted)
}

pub fn receptor_recycling(internalized: f64, recycling_rate: f64, degradation_rate: f64) -> f64 {
    recycling_rate / (recycling_rate + degradation_rate).max(1e-30) * internalized
}

pub fn mitochondrial_fission_rate(drp1: f64, fis1: f64, threshold: f64) -> f64 {
    let signal = drp1 * fis1;
    if signal > threshold {
        signal - threshold
    } else {
        0.0
    }
}

pub fn mitochondrial_fusion_rate(mfn1: f64, mfn2: f64, opa1: f64) -> f64 {
    (mfn1 + mfn2) * opa1
}

pub fn er_stress_upr(misfolded: f64, bip: f64, ire1_threshold: f64) -> f64 {
    let free_misfolded = (misfolded - bip).max(0.0);
    free_misfolded / (ire1_threshold + free_misfolded)
}

pub fn golgi_transport_rate(cargo: f64, coat_protein: f64, gtp: f64, km_coat: f64) -> f64 {
    cargo * coat_protein / (km_coat + coat_protein) * gtp
}

pub fn peroxisome_beta_oxidation(vlcfa: f64, enzyme_activity: f64, km: f64) -> f64 {
    enzyme_activity * vlcfa / (km + vlcfa)
}

pub fn cytoskeleton_treadmilling(polymerization_rate: f64, depolymerization_rate: f64) -> f64 {
    polymerization_rate - depolymerization_rate
}

pub fn nuclear_import_rate(cargo: f64, importin: f64, ran_gtp: f64, kd: f64) -> f64 {
    cargo * importin / (kd + importin) * (1.0 - ran_gtp)
}

pub fn cell_volume_regulation(
    volume: f64,
    target_volume: f64,
    permeability: f64,
    osmotic_difference: f64,
) -> f64 {
    permeability * osmotic_difference * (target_volume - volume).signum()
}
