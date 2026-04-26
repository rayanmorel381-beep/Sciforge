use sciforge::benchmark::simulation::{RenderSink, SimConfig, SimState, StepFn, run, run_headless};

struct ConstVel;

impl StepFn for ConstVel {
    fn integrate(&self, state: &mut SimState, dt: f64) {
        for i in 0..state.positions.len() {
            for d in 0..3 {
                state.positions[i][d] += state.velocities[i][d] * dt;
            }
        }
    }
}

struct FrameCounter {
    count: u64,
}

impl RenderSink for FrameCounter {
    fn frame(&mut self, _state: &SimState) {
        self.count += 1;
    }
}

#[test]
fn headless_runs_all_steps() {
    let cfg = SimConfig {
        dt: 0.01,
        max_steps: 100,
        body_count: 2,
    };
    let mut st = SimState::new(2);
    st.velocities[0] = [1.0, 0.0, 0.0];
    assert_eq!(run_headless(&cfg, &ConstVel, &mut st).total_steps, 100);
}

#[test]
fn headless_advances_position() {
    let cfg = SimConfig {
        dt: 0.1,
        max_steps: 10,
        body_count: 1,
    };
    let mut st = SimState::new(1);
    st.velocities[0] = [1.0, 0.0, 0.0];
    run_headless(&cfg, &ConstVel, &mut st);
    assert!((st.positions[0][0] - 1.0).abs() < 1e-10);
}

#[test]
fn render_sink_receives_frames() {
    let cfg = SimConfig {
        dt: 0.01,
        max_steps: 50,
        body_count: 1,
    };
    let mut st = SimState::new(1);
    let mut sink = FrameCounter { count: 0 };
    run(&cfg, &ConstVel, &mut sink, &mut st);
    assert_eq!(sink.count, 50);
}

#[test]
fn sim_state_zeros() {
    let st = SimState::new(3);
    assert_eq!(st.positions.len(), 3);
    assert_eq!(st.time, 0.0);
    assert_eq!(st.step, 0);
}

#[test]
fn zero_steps() {
    let cfg = SimConfig {
        dt: 1.0,
        max_steps: 0,
        body_count: 1,
    };
    let mut st = SimState::new(1);
    assert_eq!(run_headless(&cfg, &ConstVel, &mut st).total_steps, 0);
}

#[test]
fn time_advances() {
    let cfg = SimConfig {
        dt: 0.25,
        max_steps: 4,
        body_count: 1,
    };
    let mut st = SimState::new(1);
    run_headless(&cfg, &ConstVel, &mut st);
    assert!((st.time - 1.0).abs() < 1e-10);
}
