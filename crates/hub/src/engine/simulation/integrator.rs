use super::model::DynamicalSystem;
use crate::domain::common::errors::{HubError, HubResult};

/// Available numerical integration methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationMethod {
    Euler,
    Heun,
    RungeKutta4,
    Midpoint,
}

/// Configuration for the ODE integrator.
#[derive(Debug, Clone)]
pub struct IntegratorConfig {
    /// Integration scheme.
    pub method: IntegrationMethod,
    /// Time step.
    pub dt: f64,
    /// Maximum number of steps.
    pub max_steps: usize,
    /// Error tolerance.
    pub tolerance: f64,
}

impl IntegratorConfig {
    /// Creates a config with default max_steps and tolerance.
    pub fn new(method: IntegrationMethod, dt: f64) -> Self {
        Self {
            method,
            dt,
            max_steps: 1_000_000,
            tolerance: 1e-8,
        }
    }
}

/// Integrates the system from `t0` to `tf` starting at `y0`.
pub fn integrate(
    config: &IntegratorConfig,
    system: &dyn DynamicalSystem,
    y0: &[f64],
    t0: f64,
    tf: f64,
) -> HubResult<(Vec<f64>, Vec<Vec<f64>>)> {
    if config.dt <= 0.0 {
        return Err(HubError::InvalidInput("dt must be positive".into()));
    }
    let steps = ((tf - t0) / config.dt).ceil() as usize;
    if steps > config.max_steps {
        return Err(HubError::InvalidInput("too many steps required".into()));
    }

    let dim = y0.len();
    let mut times = Vec::with_capacity(steps + 1);
    let mut states = Vec::with_capacity(steps + 1);
    let mut y = y0.to_vec();
    let mut t = t0;
    let mut dy = vec![0.0; dim];

    times.push(t);
    states.push(y.clone());

    for _ in 0..steps {
        match config.method {
            IntegrationMethod::Euler => {
                system.derivatives(t, &y, &mut dy);
                for (yi, &dyi) in y.iter_mut().zip(dy.iter()) {
                    *yi += config.dt * dyi;
                }
            }
            IntegrationMethod::Heun => {
                system.derivatives(t, &y, &mut dy);
                let k1 = dy.clone();
                let y_pred: Vec<f64> = y
                    .iter()
                    .zip(k1.iter())
                    .map(|(&yi, &ki)| yi + config.dt * ki)
                    .collect();
                system.derivatives(t + config.dt, &y_pred, &mut dy);
                for (i, yi) in y.iter_mut().enumerate() {
                    *yi += 0.5 * config.dt * (k1[i] + dy[i]);
                }
            }
            IntegrationMethod::Midpoint => {
                system.derivatives(t, &y, &mut dy);
                let y_mid: Vec<f64> = y
                    .iter()
                    .zip(dy.iter())
                    .map(|(&yi, &ki)| yi + 0.5 * config.dt * ki)
                    .collect();
                system.derivatives(t + 0.5 * config.dt, &y_mid, &mut dy);
                for (yi, &dyi) in y.iter_mut().zip(dy.iter()) {
                    *yi += config.dt * dyi;
                }
            }
            IntegrationMethod::RungeKutta4 => {
                system.derivatives(t, &y, &mut dy);
                let k1 = dy.clone();
                let y2: Vec<f64> = y
                    .iter()
                    .zip(k1.iter())
                    .map(|(&yi, &ki)| yi + 0.5 * config.dt * ki)
                    .collect();
                system.derivatives(t + 0.5 * config.dt, &y2, &mut dy);
                let k2 = dy.clone();
                let y3: Vec<f64> = y
                    .iter()
                    .zip(k2.iter())
                    .map(|(&yi, &ki)| yi + 0.5 * config.dt * ki)
                    .collect();
                system.derivatives(t + 0.5 * config.dt, &y3, &mut dy);
                let k3 = dy.clone();
                let y4: Vec<f64> = y
                    .iter()
                    .zip(k3.iter())
                    .map(|(&yi, &ki)| yi + config.dt * ki)
                    .collect();
                system.derivatives(t + config.dt, &y4, &mut dy);
                for i in 0..dim {
                    y[i] += config.dt / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + dy[i]);
                }
            }
        }
        t += config.dt;
        times.push(t);
        states.push(y.clone());
    }

    Ok((times, states))
}
