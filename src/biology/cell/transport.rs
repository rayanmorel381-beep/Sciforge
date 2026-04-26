pub fn fick_first_law(d: f64, dc_dx: f64) -> f64 {
    -d * dc_dx
}

pub fn fick_second_law_1d(conc: &mut [f64], d: f64, dx: f64, dt: f64, steps: usize) {
    let n = conc.len();
    for _ in 0..steps {
        let old = conc.to_vec();
        for i in 1..n - 1 {
            let laplacian = (old[i + 1] - 2.0 * old[i] + old[i - 1]) / (dx * dx);
            conc[i] += d * laplacian * dt;
        }
    }
}

pub fn nernst_potential(z: f64, t: f64, c_out: f64, c_in: f64) -> f64 {
    use crate::constants::{E_CHARGE, K_B};
    let e = E_CHARGE;
    let ratio = c_out / c_in;
    if ratio <= 0.0 {
        return 0.0;
    }
    (K_B * t / (z * e)) * ratio.ln()
}

pub fn goldman_equation(
    p_na: f64,
    p_k: f64,
    p_cl: f64,
    na_out: f64,
    na_in: f64,
    k_out: f64,
    k_in: f64,
    cl_out: f64,
    cl_in: f64,
    t: f64,
) -> f64 {
    use crate::constants::{E_CHARGE, K_B};
    let e = E_CHARGE;
    let num = p_na * na_out + p_k * k_out + p_cl * cl_in;
    let den = p_na * na_in + p_k * k_in + p_cl * cl_out;
    if den < 1e-30 {
        return 0.0;
    }
    (K_B * t / e) * (num / den).ln()
}

pub fn osmotic_pressure(c: f64, t: f64) -> f64 {
    use crate::constants::{K_B, N_A};
    c * N_A * K_B * t
}

pub fn donnan_ratio(z_ion: f64, z_macro: f64, c_macro: f64, c_salt: f64) -> f64 {
    if c_salt < 1e-30 {
        return 1.0;
    }
    let r = -z_macro * c_macro / (2.0 * z_ion * c_salt);
    r + (r * r + 1.0).sqrt()
}

pub fn active_transport_rate(vmax: f64, substrate: f64, km: f64, atp: f64, km_atp: f64) -> f64 {
    vmax * substrate / (km + substrate) * atp / (km_atp + atp)
}

pub fn membrane_capacitance_current(cm: f64, dv_dt: f64) -> f64 {
    cm * dv_dt
}

pub fn electrochemical_gradient(z: f64, vm: f64, equilibrium_potential: f64) -> f64 {
    z * (vm - equilibrium_potential)
}

pub fn vesicle_fusion_rate(calcium: f64, kd: f64, n: f64, k_max: f64) -> f64 {
    k_max * calcium.powf(n) / (kd.powf(n) + calcium.powf(n))
}

pub fn endocytosis_rate(
    receptor_bound: f64,
    k_intern: f64,
    coat_protein: f64,
    kd_coat: f64,
) -> f64 {
    k_intern * receptor_bound * coat_protein / (kd_coat + coat_protein)
}

pub fn exocytosis_rate(vesicles: f64, calcium: f64, kd: f64) -> f64 {
    vesicles * calcium / (kd + calcium)
}

pub fn gap_junction_flux(c1: f64, c2: f64, permeability: f64) -> f64 {
    permeability * (c1 - c2)
}

pub fn facilitated_diffusion(c_out: f64, c_in: f64, vmax: f64, km: f64) -> f64 {
    vmax * (c_out - c_in) / (km + c_out + c_in)
}

pub fn cotransport_rate(substrate: f64, ion: f64, vmax: f64, km_s: f64, km_i: f64) -> f64 {
    vmax * substrate / (km_s + substrate) * ion / (km_i + ion)
}

pub fn pinocytosis_uptake(volume_rate: f64, concentration: f64) -> f64 {
    volume_rate * concentration
}

pub fn ion_channel_conductance(g_max: f64, open_probability: f64, driving_force: f64) -> f64 {
    g_max * open_probability * driving_force
}
