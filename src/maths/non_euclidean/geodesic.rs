use super::metric::MetricSpace;

pub struct GeodesicSolver {
    pub space: MetricSpace,
    pub epsilon: f64,
}

impl GeodesicSolver {
    pub fn new(space: MetricSpace, epsilon: f64) -> Self {
        Self { space, epsilon }
    }

    pub fn step(&self, position: &mut [f64], velocity: &mut [f64], dt: f64) {
        let gamma = self.space.christoffel_at(position, self.epsilon);
        let n = self.space.dimension();

        let mut accel = vec![0.0; n];
        for sigma in 0..n {
            for mu in 0..n {
                for nu in 0..n {
                    accel[sigma] -= gamma[sigma][mu][nu] * velocity[mu] * velocity[nu];
                }
            }
        }

        for i in 0..n {
            velocity[i] += accel[i] * dt;
            position[i] += velocity[i] * dt;
        }
    }

    pub fn integrate(
        &self,
        position: &mut [f64],
        velocity: &mut [f64],
        dt: f64,
        steps: usize,
    ) -> Vec<Vec<f64>> {
        let mut trajectory = Vec::with_capacity(steps + 1);
        trajectory.push(position.to_vec());
        for _ in 0..steps {
            self.step(position, velocity, dt);
            trajectory.push(position.to_vec());
        }
        trajectory
    }
}
