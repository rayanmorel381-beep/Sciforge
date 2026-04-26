use std::time::Instant;

#[derive(Clone, Debug)]
pub struct SimState {
    pub positions: Vec<[f64; 3]>,
    pub velocities: Vec<[f64; 3]>,
    pub time: f64,
    pub step: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct SimConfig {
    pub dt: f64,
    pub max_steps: u64,
    pub body_count: usize,
}

pub trait StepFn {
    fn integrate(&self, state: &mut SimState, dt: f64);
}

pub trait RenderSink {
    fn frame(&mut self, state: &SimState);
}

struct NullSink;

impl RenderSink for NullSink {
    fn frame(&mut self, _state: &SimState) {}
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

pub fn run<S: StepFn, R: RenderSink>(
    config: &SimConfig,
    step_fn: &S,
    sink: &mut R,
    state: &mut SimState,
) -> SimResult {
    let start = Instant::now();
    for _ in 0..config.max_steps {
        step_fn.integrate(state, config.dt);
        state.time += config.dt;
        state.step += 1;
        sink.frame(state);
    }
    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_millis() as u64;
    let avg_step_ns = if config.max_steps > 0 {
        elapsed.as_nanos() as f64 / config.max_steps as f64
    } else {
        0.0
    };
    SimResult {
        final_state: state.clone(),
        total_steps: state.step,
        elapsed_ms,
        avg_step_ns,
    }
}

pub fn run_headless<S: StepFn>(config: &SimConfig, step_fn: &S, state: &mut SimState) -> SimResult {
    run(config, step_fn, &mut NullSink, state)
}
