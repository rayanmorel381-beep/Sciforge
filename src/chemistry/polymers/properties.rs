pub fn degree_of_polymerization_number(mn: f64, m0: f64) -> f64 {
    mn / m0.max(1e-30)
}

pub fn degree_of_polymerization_weight(mw: f64, m0: f64) -> f64 {
    mw / m0.max(1e-30)
}

pub fn polydispersity_index(mw: f64, mn: f64) -> f64 {
    mw / mn.max(1e-30)
}

pub fn intrinsic_viscosity_mark_houwink(k: f64, m: f64, a: f64) -> f64 {
    k * m.powf(a)
}

pub fn end_to_end_distance_freely_jointed(n: f64, l: f64) -> f64 {
    l * n.sqrt()
}

pub fn radius_of_gyration(end_to_end: f64) -> f64 {
    end_to_end / 6.0_f64.sqrt()
}

pub fn glass_transition_fox(w1: f64, tg1: f64, w2: f64, tg2: f64) -> f64 {
    1.0 / (w1 / tg1.max(1e-30) + w2 / tg2.max(1e-30))
}

pub fn carothers_equation(p: f64, f_avg: f64) -> f64 {
    2.0 / (2.0 - p * f_avg).max(1e-30)
}
