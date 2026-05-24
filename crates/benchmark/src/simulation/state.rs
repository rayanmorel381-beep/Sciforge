#[derive(Clone, Debug)]
pub struct SimState {
    pub positions: Vec<[f64; 3]>,
    pub velocities: Vec<[f64; 3]>,
    pub time: f64,
    pub step: u64,
}

impl SimState {
    pub fn new(body_count: usize) -> Self {
        Self {
            positions: vec![[0.0; 3]; body_count],
            velocities: vec![[0.0; 3]; body_count],
            time: 0.0,
            step: 0,
        }
    }
}

pub struct SimResult {
    pub final_state: SimState,
    pub total_steps: u64,
    pub elapsed_ms: u64,
    pub avg_step_ns: f64,
}
