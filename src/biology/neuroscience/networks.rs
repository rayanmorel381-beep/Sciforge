pub struct Synapse {
    pub weight: f64,
    pub tau_rise: f64,
    pub tau_decay: f64,
    pub reversal: f64,
    pub g: f64,
    pub s: f64,
}

impl Synapse {
    pub fn excitatory(weight: f64) -> Self {
        Self {
            weight,
            tau_rise: 0.5,
            tau_decay: 5.0,
            reversal: 0.0,
            g: 0.0,
            s: 0.0,
        }
    }

    pub fn inhibitory(weight: f64) -> Self {
        Self {
            weight,
            tau_rise: 1.0,
            tau_decay: 10.0,
            reversal: -80.0,
            g: 0.0,
            s: 0.0,
        }
    }

    pub fn step(&mut self, pre_spike: bool, dt: f64) {
        if pre_spike {
            self.s += self.weight;
        }
        let ds = -self.s / self.tau_rise;
        let dg = (self.s - self.g) / self.tau_decay;
        self.s += ds * dt;
        self.g += dg * dt;
        self.s = self.s.max(0.0);
        self.g = self.g.max(0.0);
    }

    pub fn current(&self, v_post: f64) -> f64 {
        self.g * (v_post - self.reversal)
    }
}

pub fn stdp_update(delta_t: f64, a_plus: f64, a_minus: f64, tau_plus: f64, tau_minus: f64) -> f64 {
    if delta_t > 0.0 {
        a_plus * (-delta_t / tau_plus).exp()
    } else {
        -a_minus * (delta_t / tau_minus).exp()
    }
}

pub fn simulate_network(
    n_neurons: usize,
    weights: &[Vec<f64>],
    external_current: f64,
    dt: f64,
    steps: usize,
    threshold: f64,
    reset: f64,
    tau: f64,
    resistance: f64,
    rest: f64,
) -> Vec<Vec<f64>> {
    let mut voltages = vec![rest; n_neurons];
    let mut traces = vec![Vec::with_capacity(steps + 1); n_neurons];
    for i in 0..n_neurons {
        traces[i].push(voltages[i]);
    }

    for _ in 0..steps {
        let mut spikes = vec![false; n_neurons];
        for i in 0..n_neurons {
            if voltages[i] >= threshold {
                spikes[i] = true;
                voltages[i] = reset;
            }
        }

        let old_v = voltages.clone();
        for i in 0..n_neurons {
            let mut syn_input = 0.0;
            for j in 0..n_neurons {
                if spikes[j] {
                    syn_input += weights[j][i];
                }
            }
            let dv = (-(old_v[i] - rest) + resistance * (external_current + syn_input)) / tau;
            voltages[i] += dv * dt;
            traces[i].push(voltages[i]);
        }
    }
    traces
}

pub fn mean_field_rate(mu: f64, sigma: f64, threshold: f64, reset: f64, tau: f64) -> f64 {
    let sqrt_pi = core::f64::consts::PI.sqrt();
    let lower = (reset - mu) / sigma;
    let upper = (threshold - mu) / sigma;
    let n_steps = 100;
    let dx = (upper - lower) / n_steps as f64;
    let mut integral = 0.0;
    for i in 0..n_steps {
        let x = lower + (i as f64 + 0.5) * dx;
        integral += (x * x).exp() * (1.0 + erf_approx(x)) * dx;
    }
    1.0 / (tau * sqrt_pi * integral + 0.002 * tau)
}

fn erf_approx(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.3275911 * x.abs());
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let result = 1.0 - poly * (-x * x).exp();
    if x >= 0.0 { result } else { -result }
}
