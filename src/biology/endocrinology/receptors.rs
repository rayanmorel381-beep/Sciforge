pub fn receptor_binding_fraction(ligand: f64, kd: f64) -> f64 {
    ligand / (ligand + kd)
}

pub fn competitive_binding(ligand: f64, competitor: f64, kd: f64, ki: f64) -> f64 {
    ligand / (ligand + kd * (1.0 + competitor / ki))
}

pub fn receptor_up_regulation(r0: f64, stimulus: f64, k_up: f64, k_deg: f64, t: f64) -> f64 {
    let r_ss = (r0 * k_deg + k_up * stimulus) / k_deg;
    r_ss - (r_ss - r0) * (-k_deg * t).exp()
}

pub fn receptor_down_regulation(r0: f64, stimulus: f64, k_down: f64, k_synth: f64, t: f64) -> f64 {
    let effective_deg = k_down * stimulus + k_down;
    let r_ss = k_synth / effective_deg;
    r_ss + (r0 - r_ss) * (-effective_deg * t).exp()
}

pub fn dose_response_hill(dose: f64, ec50: f64, emax: f64, n: f64) -> f64 {
    emax * dose.powf(n) / (ec50.powf(n) + dose.powf(n))
}

pub fn insulin_glucose_minimal_model(
    g: f64,
    x: f64,
    insulin: f64,
    gb: f64,
    p1: f64,
    p2: f64,
    p3: f64,
    ib: f64,
) -> (f64, f64) {
    let dg = -p1 * (g - gb) - x * g;
    let dx = -p2 * x + p3 * (insulin - ib);
    (dg, dx)
}

pub fn receptor_internalization(
    surface: f64,
    ligand: f64,
    k_intern: f64,
    k_recycle: f64,
) -> (f64, f64) {
    let internalized = k_intern * surface * ligand;
    let recycled = k_recycle * (1.0 - surface);
    (-internalized + recycled, internalized - recycled)
}

pub fn receptor_clearance_rate(concentration: f64, half_life: f64) -> f64 {
    concentration * std::f64::consts::LN_2 / half_life
}

pub fn feedback_loop_negative(
    stimulus: f64,
    hormone: f64,
    sensitivity: f64,
    set_point: f64,
) -> f64 {
    sensitivity * (set_point - hormone) * stimulus
}

pub fn receptor_pulsatile_response(amplitude: f64, frequency: f64, t: f64, baseline: f64) -> f64 {
    baseline + amplitude * (0.5 * (1.0 + (2.0 * std::f64::consts::PI * frequency * t).sin()))
}

pub fn allosteric_modulation(ligand: f64, modulator: f64, kd: f64, alpha: f64, beta: f64) -> f64 {
    let effective_kd = kd / (1.0 + alpha * modulator);
    beta * ligand / (ligand + effective_kd)
}

pub fn spare_receptor_response(ligand: f64, kd: f64, receptor_reserve: f64) -> f64 {
    let occupancy = ligand / (ligand + kd);
    let amplified = occupancy * receptor_reserve;
    amplified / (1.0 + amplified)
}

pub fn desensitization_kinetics(
    r0: f64,
    agonist: f64,
    k_desens: f64,
    k_resens: f64,
    t: f64,
) -> f64 {
    let r_ss = r0 * k_resens / (k_resens + k_desens * agonist);
    r_ss + (r0 - r_ss) * (-(k_resens + k_desens * agonist) * t).exp()
}

pub fn second_messenger_camp(receptor_activity: f64, k_synth: f64, k_pde: f64, basal: f64) -> f64 {
    basal + k_synth * receptor_activity / k_pde
}

pub fn ip3_calcium_release(ip3: f64, k_release: f64, k_serca: f64, store: f64) -> f64 {
    k_release * ip3 * store / (ip3 + k_release) - k_serca * store
}

pub fn receptor_dimerization(monomer: f64, kd_dimer: f64) -> f64 {
    let discriminant = (monomer + kd_dimer).powi(2) - monomer.powi(2);
    0.5 * ((monomer + kd_dimer) - discriminant.max(0.0).sqrt())
}

pub fn beta_arrestin_recruitment(agonist: f64, receptor: f64, k_arr: f64) -> f64 {
    k_arr * agonist * receptor / (1.0 + agonist)
}

pub fn receptor_tyrosine_kinase_activation(ligand: f64, receptor: f64, km: f64, vmax: f64) -> f64 {
    let bound = receptor * ligand / (ligand + km);
    vmax * bound / receptor.max(1e-30)
}

pub fn gpcr_g_protein_cycle(
    active_receptor: f64,
    gdp_bound: f64,
    k_exchange: f64,
    k_hydrolysis: f64,
) -> (f64, f64) {
    let activation = k_exchange * active_receptor * gdp_bound;
    let deactivation = k_hydrolysis * (1.0 - gdp_bound);
    (activation - deactivation, -(activation - deactivation))
}
