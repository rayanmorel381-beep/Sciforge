mod config;
mod state;
mod traits;

pub use config::SimConfig;
pub use state::{SimResult, SimState};
pub use traits::{RenderSink, StepFn};

use std::time::Instant;

struct NullSink;

impl RenderSink for NullSink {
    fn frame(&mut self, _state: &SimState) {}
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
