use std::f64::consts::PI;

pub struct RlcCircuit {
    pub r: f64,
    pub l: f64,
    pub c: f64,
}

impl RlcCircuit {
    pub fn new(r: f64, l: f64, c: f64) -> Self {
        Self { r, l, c }
    }

    pub fn resonant_frequency(&self) -> f64 {
        1.0 / (2.0 * PI * (self.l * self.c).sqrt())
    }

    pub fn quality_factor(&self) -> f64 {
        (1.0 / self.r) * (self.l / self.c).sqrt()
    }

    pub fn impedance(&self, omega: f64) -> (f64, f64) {
        let xl = omega * self.l;
        let xc = 1.0 / (omega * self.c);
        let z_real = self.r;
        let z_imag = xl - xc;
        (z_real, z_imag)
    }

    pub fn impedance_magnitude(&self, omega: f64) -> f64 {
        let (zr, zi) = self.impedance(omega);
        (zr * zr + zi * zi).sqrt()
    }

    pub fn phase_angle(&self, omega: f64) -> f64 {
        let (zr, zi) = self.impedance(omega);
        zi.atan2(zr)
    }

    pub fn damping_ratio(&self) -> f64 {
        self.r / (2.0 * (self.l / self.c).sqrt())
    }

    pub fn bandwidth(&self) -> f64 {
        self.r / self.l
    }

    pub fn transient_response(&self, t: f64, v0: f64) -> f64 {
        let alpha = self.r / (2.0 * self.l);
        let omega0 = 1.0 / (self.l * self.c).sqrt();
        if alpha < omega0 {
            let omega_d = (omega0 * omega0 - alpha * alpha).sqrt();
            v0 * (-alpha * t).exp() * (omega_d * t).cos()
        } else if alpha > omega0 {
            let s1 = -alpha + (alpha * alpha - omega0 * omega0).sqrt();
            let s2 = -alpha - (alpha * alpha - omega0 * omega0).sqrt();
            v0 * (s2 * (s1 * t).exp() - s1 * (s2 * t).exp()) / (s2 - s1)
        } else {
            v0 * (1.0 + alpha * t) * (-alpha * t).exp()
        }
    }
}

pub fn rc_time_constant(r: f64, c: f64) -> f64 {
    r * c
}

pub fn rl_time_constant(r: f64, l: f64) -> f64 {
    l / r
}

pub fn rc_charging(v_source: f64, r: f64, c: f64, t: f64) -> f64 {
    v_source * (1.0 - (-t / (r * c)).exp())
}

pub fn rc_discharging(v0: f64, r: f64, c: f64, t: f64) -> f64 {
    v0 * (-t / (r * c)).exp()
}

pub fn power_dissipated(v: f64, r: f64) -> f64 {
    v * v / r
}

pub fn parallel_resistance(resistances: &[f64]) -> f64 {
    1.0 / resistances.iter().map(|&r| 1.0 / r).sum::<f64>()
}

pub fn series_resistance(resistances: &[f64]) -> f64 {
    resistances.iter().sum()
}

pub fn parallel_capacitance(capacitances: &[f64]) -> f64 {
    capacitances.iter().sum()
}

pub fn series_capacitance(capacitances: &[f64]) -> f64 {
    1.0 / capacitances.iter().map(|&c| 1.0 / c).sum::<f64>()
}

pub fn voltage_divider(v_in: f64, r1: f64, r2: f64) -> f64 {
    v_in * r2 / (r1 + r2)
}

pub fn wheatstone_bridge_balance(r1: f64, r2: f64, r3: f64) -> f64 {
    r2 * r3 / r1
}

pub fn energy_capacitor(c: f64, v: f64) -> f64 {
    0.5 * c * v * v
}

pub fn energy_inductor(l: f64, i: f64) -> f64 {
    0.5 * l * i * i
}

pub fn mutual_inductance_coupling(k: f64, l1: f64, l2: f64) -> f64 {
    k * (l1 * l2).sqrt()
}

pub fn transformer_ratio(n_primary: f64, n_secondary: f64, v_primary: f64) -> f64 {
    v_primary * n_secondary / n_primary
}
