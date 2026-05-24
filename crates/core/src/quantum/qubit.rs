use sciforge_hub::prelude::maths::complex::Complex;
use sciforge_hub::prelude::physics::quantum as q;

#[derive(Debug, Clone)]
pub struct Qubit {
    pub state: [Complex; 2],
}

impl Qubit {
    pub fn new(state: [Complex; 2]) -> Self {
        Self { state }
    }

    pub fn zero() -> Self {
        let up = q::spin_up();
        Self { state: [up[0], up[1]] }
    }

    pub fn one() -> Self {
        let down = q::spin_down();
        Self { state: [down[0], down[1]] }
    }

    pub fn plus_x() -> Self {
        let s = q::spin_plus_x();
        Self { state: [s[0], s[1]] }
    }

    pub fn minus_x() -> Self {
        let s = q::spin_minus_x();
        Self { state: [s[0], s[1]] }
    }

    pub fn plus_y() -> Self {
        let s = q::spin_plus_y();
        Self { state: [s[0], s[1]] }
    }

    pub fn minus_y() -> Self {
        let s = q::spin_minus_y();
        Self { state: [s[0], s[1]] }
    }

    pub fn bloch_vector(&self) -> (f64, f64, f64) {
        q::bloch_vector(&self.state)
    }

    pub fn measure_probabilities(&self) -> Vec<f64> {
        q::measure_probabilities(&self.state)
    }

    pub fn apply(&self, gate: &[Vec<Complex>]) -> Self {
        let new_state = q::apply_gate(gate, &self.state);
        Self {
            state: [new_state[0], new_state[1]],
        }
    }
}
