use super::state::SimState;

pub trait StepFn {
    fn integrate(&self, state: &mut SimState, dt: f64);
}

pub trait RenderSink {
    fn frame(&mut self, state: &SimState);
}
