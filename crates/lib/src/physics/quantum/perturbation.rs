use crate::constants::HBAR;
use crate::maths::complex::Complex;

pub fn first_order_energy(v_matrix: &[Vec<Complex>], state: &[Complex]) -> f64 {
    let mut result = Complex::zero();
    for (vi_row, &si) in v_matrix.iter().zip(state.iter()) {
        let mut vi_psi = Complex::zero();
        for (&vij, &sj) in vi_row.iter().zip(state.iter()) {
            vi_psi = vi_psi + vij * sj;
        }
        result = result + si.conj() * vi_psi;
    }
    result.re
}

pub fn second_order_energy(
    h0_energies: &[f64],
    v_matrix: &[Vec<Complex>],
    states: &[Vec<Complex>],
    n_index: usize,
) -> f64 {
    let dim = states.len();
    let en = h0_energies[n_index];
    let state_n = &states[n_index];
    let mut correction = 0.0;

    for k in 0..dim {
        if k == n_index {
            continue;
        }
        let ek = h0_energies[k];
        let de = en - ek;
        if de.abs() < 1e-30 {
            continue;
        }

        let mut vnk = Complex::zero();
        for (i, &ski) in states[k].iter().enumerate() {
            let mut v_row = Complex::zero();
            for (&vij, &snj) in v_matrix[i].iter().zip(state_n.iter()) {
                v_row = v_row + vij * snj;
            }
            vnk = vnk + ski.conj() * v_row;
        }
        correction += vnk.norm_sq() / de;
    }
    correction
}

pub fn first_order_state(
    h0_energies: &[f64],
    v_matrix: &[Vec<Complex>],
    states: &[Vec<Complex>],
    n_index: usize,
) -> Vec<Complex> {
    let dim = states[0].len();
    let n_states = states.len();
    let en = h0_energies[n_index];
    let state_n = &states[n_index];
    let mut correction = vec![Complex::zero(); dim];

    for k in 0..n_states {
        if k == n_index {
            continue;
        }
        let ek = h0_energies[k];
        let de = en - ek;
        if de.abs() < 1e-30 {
            continue;
        }

        let mut vnk = Complex::zero();
        for (i, &ski) in states[k].iter().enumerate() {
            let mut v_row = Complex::zero();
            for (&vij, &snj) in v_matrix[i].iter().zip(state_n.iter()) {
                v_row = v_row + vij * snj;
            }
            vnk = vnk + ski.conj() * v_row;
        }

        let coeff = vnk * Complex::new(1.0 / de, 0.0);
        for (ci, &ski) in correction.iter_mut().zip(states[k].iter()) {
            *ci = *ci + ski * coeff;
        }
    }
    correction
}

pub fn variational_energy(hamiltonian: &[Vec<Complex>], trial: &[Complex]) -> f64 {
    let n = trial.len();
    let mut h_psi = vec![Complex::zero(); n];
    for (hp, hi_row) in h_psi.iter_mut().zip(hamiltonian.iter()) {
        for (&hij, &tj) in hi_row.iter().zip(trial.iter()) {
            *hp = *hp + hij * tj;
        }
    }

    let mut numerator = Complex::zero();
    let mut denominator = Complex::zero();
    for (&ti, &hp) in trial.iter().zip(h_psi.iter()) {
        numerator = numerator + ti.conj() * hp;
        denominator = denominator + ti.conj() * ti;
    }
    numerator.re / denominator.re
}

pub fn variational_minimize(
    hamiltonian: &[Vec<Complex>],
    trial_fn: impl Fn(f64) -> Vec<Complex>,
    param_min: f64,
    param_max: f64,
    n_points: usize,
) -> (f64, f64) {
    let mut best_param = param_min;
    let mut best_energy = f64::INFINITY;
    for i in 0..n_points {
        let p = param_min + (param_max - param_min) * i as f64 / (n_points - 1) as f64;
        let trial = trial_fn(p);
        let e = variational_energy(hamiltonian, &trial);
        if e < best_energy {
            best_energy = e;
            best_param = p;
        }
    }

    let dp = (param_max - param_min) / (n_points - 1) as f64;
    let lo = best_param - dp;
    let hi = best_param + dp;
    let mut a = lo;
    let mut b = hi;
    for _ in 0..60 {
        let m1 = a + (b - a) / 3.0;
        let m2 = b - (b - a) / 3.0;
        let e1 = variational_energy(hamiltonian, &trial_fn(m1));
        let e2 = variational_energy(hamiltonian, &trial_fn(m2));
        if e1 < e2 {
            b = m2;
        } else {
            a = m1;
        }
    }
    let optimal = (a + b) / 2.0;
    (optimal, variational_energy(hamiltonian, &trial_fn(optimal)))
}

pub fn wkb_phase(v: impl Fn(f64) -> f64, energy: f64, x1: f64, x2: f64, mass: f64) -> f64 {
    let n = 10000;
    let dx = (x2 - x1) / n as f64;
    let mut sum = 0.0;
    let integrand = |x: f64| {
        let diff = energy - v(x);
        if diff > 0.0 {
            (2.0 * mass * diff).sqrt()
        } else {
            0.0
        }
    };
    sum += integrand(x1) + integrand(x2);
    for i in 1..n {
        let x = x1 + i as f64 * dx;
        let w = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += w * integrand(x);
    }
    sum * dx / (3.0 * HBAR)
}

pub fn wkb_transmission(v: impl Fn(f64) -> f64, energy: f64, x1: f64, x2: f64, mass: f64) -> f64 {
    let n = 10000;
    let dx = (x2 - x1) / n as f64;
    let mut sum = 0.0;
    let integrand = |x: f64| {
        let diff = v(x) - energy;
        if diff > 0.0 {
            (2.0 * mass * diff).sqrt()
        } else {
            0.0
        }
    };
    sum += integrand(x1) + integrand(x2);
    for i in 1..n {
        let x = x1 + i as f64 * dx;
        let w = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += w * integrand(x);
    }
    let gamma = sum * dx / (3.0 * HBAR);
    (-2.0 * gamma).exp()
}

pub fn wkb_quantization(
    v: impl Fn(f64) -> f64,
    mass: f64,
    x_min: f64,
    x_max: f64,
    n_quantum: u32,
) -> f64 {
    let target = (n_quantum as f64 + 0.5) * std::f64::consts::PI * HBAR;
    let mut e_lo = v(x_min).min(v(x_max));
    let mut e_hi = v(x_min).max(v(x_max));

    for _ in 0..100 {
        let e_mid = (e_lo + e_hi) / 2.0;
        let phase = classical_action(&v, e_mid, x_min, x_max, mass);
        if phase < target {
            e_lo = e_mid;
        } else {
            e_hi = e_mid;
        }
    }
    (e_lo + e_hi) / 2.0
}

fn classical_action(
    v: &impl Fn(f64) -> f64,
    energy: f64,
    x_min: f64,
    x_max: f64,
    mass: f64,
) -> f64 {
    let n = 5000;
    let dx = (x_max - x_min) / n as f64;
    let mut sum = 0.0;
    for i in 0..=n {
        let x = x_min + i as f64 * dx;
        let diff = energy - v(x);
        if diff > 0.0 {
            let w = if i == 0 || i == n {
                1.0
            } else if i % 2 == 0 {
                2.0
            } else {
                4.0
            };
            sum += w * (2.0 * mass * diff).sqrt();
        }
    }
    sum * dx / 3.0
}

pub fn fermi_golden_rule(
    v_matrix: &[Vec<Complex>],
    initial: &[Complex],
    finals: &[Vec<Complex>],
    density_of_states: f64,
) -> f64 {
    let mut rate = 0.0;
    for f_state in finals {
        let mut matrix_elem = Complex::zero();
        for (i, &fsi) in f_state.iter().enumerate() {
            let mut v_row = Complex::zero();
            for (&vij, &ij) in v_matrix[i].iter().zip(initial.iter()) {
                v_row = v_row + vij * ij;
            }
            matrix_elem = matrix_elem + fsi.conj() * v_row;
        }
        rate += matrix_elem.norm_sq();
    }
    2.0 * std::f64::consts::PI / HBAR * rate * density_of_states
}
