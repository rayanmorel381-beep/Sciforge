/// Trait for systems of ordinary differential equations.
pub trait DynamicalSystem {
    /// Number of state variables.
    fn dimension(&self) -> usize;
    /// Computes derivatives at time `t` and `state`, writing into `out`.
    fn derivatives(&self, t: f64, state: &[f64], out: &mut [f64]);
    /// Returns the initial state vector.
    fn initial_state(&self) -> Vec<f64>;
    /// Returns the integration time span (t0, tf).
    fn time_span(&self) -> (f64, f64);
}

/// Boxed derivative function signature.
pub type SystemFn = Box<dyn Fn(f64, &[f64], &mut [f64])>;

/// Generic model built from a closure.
pub struct SimpleModel {
    y0: Vec<f64>,
    t_span: (f64, f64),
    f: SystemFn,
}

impl SimpleModel {
    /// Creates a model from initial state, time span, and derivative function.
    pub fn new(y0: Vec<f64>, t_span: (f64, f64), f: SystemFn) -> Self {
        Self { y0, t_span, f }
    }
}

impl DynamicalSystem for SimpleModel {
    fn dimension(&self) -> usize {
        self.y0.len()
    }
    fn derivatives(&self, t: f64, state: &[f64], out: &mut [f64]) {
        (self.f)(t, state, out);
    }
    fn initial_state(&self) -> Vec<f64> {
        self.y0.clone()
    }
    fn time_span(&self) -> (f64, f64) {
        self.t_span
    }
}

/// Simple harmonic oscillator model.
pub struct HarmonicOscillator {
    /// Angular frequency.
    pub omega: f64,
    /// Initial displacement.
    pub x0: f64,
    /// Initial velocity.
    pub v0: f64,
    /// Integration time span.
    pub t_span: (f64, f64),
}

impl DynamicalSystem for HarmonicOscillator {
    fn dimension(&self) -> usize {
        2
    }
    fn derivatives(&self, _t: f64, state: &[f64], out: &mut [f64]) {
        out[0] = state[1];
        out[1] = -self.omega * self.omega * state[0];
    }
    fn initial_state(&self) -> Vec<f64> {
        vec![self.x0, self.v0]
    }
    fn time_span(&self) -> (f64, f64) {
        self.t_span
    }
}

/// Lotka-Volterra predator-prey model.
pub struct LotkaVolterra {
    /// Prey growth rate.
    pub alpha: f64,
    /// Predation rate.
    pub beta: f64,
    /// Predator reproduction rate per prey consumed.
    pub delta: f64,
    /// Predator mortality rate.
    pub gamma: f64,
    /// Initial prey population.
    pub prey0: f64,
    /// Initial predator population.
    pub pred0: f64,
    /// Integration time span.
    pub t_span: (f64, f64),
}

impl DynamicalSystem for LotkaVolterra {
    fn dimension(&self) -> usize {
        2
    }
    fn derivatives(&self, _t: f64, state: &[f64], out: &mut [f64]) {
        let x = state[0];
        let y = state[1];
        out[0] = self.alpha * x - self.beta * x * y;
        out[1] = self.delta * x * y - self.gamma * y;
    }
    fn initial_state(&self) -> Vec<f64> {
        vec![self.prey0, self.pred0]
    }
    fn time_span(&self) -> (f64, f64) {
        self.t_span
    }
}
