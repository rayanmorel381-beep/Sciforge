use crate::constants::{
    HH_ALPHA_H_COEFF, HH_ALPHA_H_EXP_COEFF, HH_ALPHA_M_COEFF, HH_ALPHA_M_V_SHIFT, HH_ALPHA_N_COEFF,
    HH_ALPHA_N_V_SHIFT, HH_BETA_H_EXP_COEFF, HH_BETA_H_V_SHIFT, HH_BETA_M_EXP_COEFF,
    HH_BETA_M_MULTIPLIER, HH_BETA_N_COEFF, HH_BETA_N_EXP_COEFF, HH_E_K, HH_E_L, HH_E_NA, HH_G_K,
    HH_G_L, HH_G_NA, HH_GATING_V_SHIFT, HH_INIT_H, HH_INIT_M, HH_INIT_N, HH_MEMBRANE_CAPACITANCE,
    HH_RESTING_POTENTIAL, IZHI_BURST_A, IZHI_BURST_B, IZHI_BURST_C, IZHI_BURST_D,
    IZHI_CONSTANT_COEFF, IZHI_FS_A, IZHI_FS_B, IZHI_FS_C, IZHI_FS_D, IZHI_INIT_U,
    IZHI_LINEAR_COEFF, IZHI_QUAD_COEFF, IZHI_RESTING_POTENTIAL, IZHI_RS_A, IZHI_RS_B, IZHI_RS_C,
    IZHI_RS_D, IZHI_SPIKE_THRESHOLD, LIF_RESISTANCE, LIF_RESTING_POTENTIAL, LIF_TAU, LIF_THRESHOLD,
    ML_CM, ML_E_CA, ML_E_K, ML_E_L, ML_G_CA, ML_G_K, ML_G_L, ML_INIT_V, ML_PHI, ML_V1, ML_V2,
    ML_V3, ML_V4,
};

pub struct HodgkinHuxley {
    pub v: f64,
    pub m: f64,
    pub h: f64,
    pub n: f64,
    pub cm: f64,
    pub g_na: f64,
    pub g_k: f64,
    pub g_l: f64,
    pub e_na: f64,
    pub e_k: f64,
    pub e_l: f64,
}

impl Default for HodgkinHuxley {
    fn default() -> Self {
        Self {
            v: HH_RESTING_POTENTIAL,
            m: HH_INIT_M,
            h: HH_INIT_H,
            n: HH_INIT_N,
            cm: HH_MEMBRANE_CAPACITANCE,
            g_na: HH_G_NA,
            g_k: HH_G_K,
            g_l: HH_G_L,
            e_na: HH_E_NA,
            e_k: HH_E_K,
            e_l: HH_E_L,
        }
    }
}

impl HodgkinHuxley {
    fn alpha_m(v: f64) -> f64 {
        if (v + HH_ALPHA_M_V_SHIFT).abs() < 1e-6 {
            return 1.0;
        }
        HH_ALPHA_M_COEFF * (v + HH_ALPHA_M_V_SHIFT)
            / (1.0 - (-HH_ALPHA_M_COEFF * (v + HH_ALPHA_M_V_SHIFT)).exp())
    }
    fn beta_m(v: f64) -> f64 {
        HH_BETA_M_MULTIPLIER * (-HH_BETA_M_EXP_COEFF * (v + HH_GATING_V_SHIFT)).exp()
    }
    fn alpha_h(v: f64) -> f64 {
        HH_ALPHA_H_COEFF * (-HH_ALPHA_H_EXP_COEFF * (v + HH_GATING_V_SHIFT)).exp()
    }
    fn beta_h(v: f64) -> f64 {
        1.0 / (1.0 + (-HH_BETA_H_EXP_COEFF * (v + HH_BETA_H_V_SHIFT)).exp())
    }
    fn alpha_n(v: f64) -> f64 {
        if (v + HH_ALPHA_N_V_SHIFT).abs() < 1e-6 {
            return HH_ALPHA_M_COEFF;
        }
        HH_ALPHA_N_COEFF * (v + HH_ALPHA_N_V_SHIFT)
            / (1.0 - (-HH_BETA_H_EXP_COEFF * (v + HH_ALPHA_N_V_SHIFT)).exp())
    }
    fn beta_n(v: f64) -> f64 {
        HH_BETA_N_COEFF * (-HH_BETA_N_EXP_COEFF * (v + HH_GATING_V_SHIFT)).exp()
    }

    pub fn step(&mut self, i_ext: f64, dt: f64) {
        let am = Self::alpha_m(self.v);
        let bm = Self::beta_m(self.v);
        let ah = Self::alpha_h(self.v);
        let bh = Self::beta_h(self.v);
        let an = Self::alpha_n(self.v);
        let bn = Self::beta_n(self.v);

        let i_na = self.g_na * self.m.powi(3) * self.h * (self.v - self.e_na);
        let i_k = self.g_k * self.n.powi(4) * (self.v - self.e_k);
        let i_l = self.g_l * (self.v - self.e_l);

        let dv = (i_ext - i_na - i_k - i_l) / self.cm;
        let dm = am * (1.0 - self.m) - bm * self.m;
        let dh = ah * (1.0 - self.h) - bh * self.h;
        let dn = an * (1.0 - self.n) - bn * self.n;

        self.v += dv * dt;
        self.m += dm * dt;
        self.h += dh * dt;
        self.n += dn * dt;
    }

    pub fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) -> Vec<f64> {
        let mut trace = Vec::with_capacity(steps + 1);
        trace.push(self.v);
        for _ in 0..steps {
            self.step(i_ext, dt);
            trace.push(self.v);
        }
        trace
    }
}

pub struct FitzHughNagumo {
    pub v: f64,
    pub w: f64,
    pub a: f64,
    pub b: f64,
    pub tau: f64,
}

impl FitzHughNagumo {
    pub fn new(a: f64, b: f64, tau: f64) -> Self {
        Self {
            v: 0.0,
            w: 0.0,
            a,
            b,
            tau,
        }
    }

    pub fn step(&mut self, i_ext: f64, dt: f64) {
        let dv = self.v - self.v.powi(3) / 3.0 - self.w + i_ext;
        let dw = (self.v + self.a - self.b * self.w) / self.tau;
        self.v += dv * dt;
        self.w += dw * dt;
    }

    pub fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut trace = Vec::with_capacity(steps + 1);
        trace.push((self.v, self.w));
        for _ in 0..steps {
            self.step(i_ext, dt);
            trace.push((self.v, self.w));
        }
        trace
    }
}

pub struct LeakyIntegrateFire {
    pub v: f64,
    pub v_rest: f64,
    pub v_thresh: f64,
    pub v_reset: f64,
    pub tau: f64,
    pub r: f64,
}

impl Default for LeakyIntegrateFire {
    fn default() -> Self {
        Self::new(
            LIF_RESTING_POTENTIAL,
            LIF_THRESHOLD,
            LIF_RESTING_POTENTIAL,
            LIF_TAU,
            LIF_RESISTANCE,
        )
    }
}

impl LeakyIntegrateFire {
    pub fn new(v_rest: f64, v_thresh: f64, v_reset: f64, tau: f64, r: f64) -> Self {
        Self {
            v: v_rest,
            v_rest,
            v_thresh,
            v_reset,
            tau,
            r,
        }
    }

    pub fn step(&mut self, i_ext: f64, dt: f64) -> bool {
        let dv = (-(self.v - self.v_rest) + self.r * i_ext) / self.tau;
        self.v += dv * dt;
        if self.v >= self.v_thresh {
            self.v = self.v_reset;
            true
        } else {
            false
        }
    }

    pub fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) -> (Vec<f64>, Vec<usize>) {
        let mut trace = Vec::with_capacity(steps + 1);
        let mut spikes = Vec::new();
        trace.push(self.v);
        for step in 0..steps {
            if self.step(i_ext, dt) {
                spikes.push(step);
            }
            trace.push(self.v);
        }
        (trace, spikes)
    }
}

pub struct IzhikevichNeuron {
    pub v: f64,
    pub u: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl IzhikevichNeuron {
    pub fn regular_spiking() -> Self {
        Self {
            v: IZHI_RESTING_POTENTIAL,
            u: IZHI_INIT_U,
            a: IZHI_RS_A,
            b: IZHI_RS_B,
            c: IZHI_RS_C,
            d: IZHI_RS_D,
        }
    }

    pub fn fast_spiking() -> Self {
        Self {
            v: IZHI_RESTING_POTENTIAL,
            u: IZHI_INIT_U,
            a: IZHI_FS_A,
            b: IZHI_FS_B,
            c: IZHI_FS_C,
            d: IZHI_FS_D,
        }
    }

    pub fn bursting() -> Self {
        Self {
            v: IZHI_RESTING_POTENTIAL,
            u: IZHI_INIT_U,
            a: IZHI_BURST_A,
            b: IZHI_BURST_B,
            c: IZHI_BURST_C,
            d: IZHI_BURST_D,
        }
    }

    pub fn step(&mut self, i_ext: f64, dt: f64) -> bool {
        let dv =
            IZHI_QUAD_COEFF * self.v * self.v + IZHI_LINEAR_COEFF * self.v + IZHI_CONSTANT_COEFF
                - self.u
                + i_ext;
        let du = self.a * (self.b * self.v - self.u);
        self.v += dv * dt;
        self.u += du * dt;
        if self.v >= IZHI_SPIKE_THRESHOLD {
            self.v = self.c;
            self.u += self.d;
            true
        } else {
            false
        }
    }

    pub fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) -> (Vec<f64>, Vec<usize>) {
        let mut trace = Vec::with_capacity(steps + 1);
        let mut spikes = Vec::new();
        trace.push(self.v);
        for step in 0..steps {
            if self.step(i_ext, dt) {
                spikes.push(step);
            }
            trace.push(self.v);
        }
        (trace, spikes)
    }
}

pub struct MorrisLecar {
    pub v: f64,
    pub w: f64,
    pub cm: f64,
    pub g_ca: f64,
    pub g_k: f64,
    pub g_l: f64,
    pub e_ca: f64,
    pub e_k: f64,
    pub e_l: f64,
    pub v1: f64,
    pub v2: f64,
    pub v3: f64,
    pub v4: f64,
    pub phi: f64,
}

impl Default for MorrisLecar {
    fn default() -> Self {
        Self {
            v: ML_INIT_V,
            w: 0.0,
            cm: ML_CM,
            g_ca: ML_G_CA,
            g_k: ML_G_K,
            g_l: ML_G_L,
            e_ca: ML_E_CA,
            e_k: ML_E_K,
            e_l: ML_E_L,
            v1: ML_V1,
            v2: ML_V2,
            v3: ML_V3,
            v4: ML_V4,
            phi: ML_PHI,
        }
    }
}

impl MorrisLecar {
    pub fn step(&mut self, i_ext: f64, dt: f64) {
        let m_ss = 0.5 * (1.0 + ((self.v - self.v1) / self.v2).tanh());
        let w_ss = 0.5 * (1.0 + ((self.v - self.v3) / self.v4).tanh());
        let tau_w = 1.0 / (self.phi * ((self.v - self.v3) / (2.0 * self.v4)).cosh());

        let i_ca = self.g_ca * m_ss * (self.v - self.e_ca);
        let i_k = self.g_k * self.w * (self.v - self.e_k);
        let i_l = self.g_l * (self.v - self.e_l);

        let dv = (i_ext - i_ca - i_k - i_l) / self.cm;
        let dw = (w_ss - self.w) / tau_w;

        self.v += dv * dt;
        self.w += dw * dt;
    }

    pub fn simulate(&mut self, i_ext: f64, dt: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut trace = Vec::with_capacity(steps + 1);
        trace.push((self.v, self.w));
        for _ in 0..steps {
            self.step(i_ext, dt);
            trace.push((self.v, self.w));
        }
        trace
    }
}
