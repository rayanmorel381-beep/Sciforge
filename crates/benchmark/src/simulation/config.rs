#[derive(Clone, Copy, Debug)]
pub struct SimConfig {
    pub dt: f64,
    pub max_steps: u64,
    pub body_count: usize,
}
