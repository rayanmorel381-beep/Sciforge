use super::fields::{gravitational_force, lorentz_force};
use super::types::{Particle, Vec3};

pub struct VectorFieldSim {
    pub particles: Vec<Particle>,
    pub dt: f64,
}

impl VectorFieldSim {
    pub fn new(dt: f64) -> Self {
        Self {
            particles: Vec::new(),
            dt,
        }
    }

    pub fn add_particle(&mut self, p: Particle) {
        self.particles.push(p);
    }

    pub fn step_gravity(&mut self) {
        let n = self.particles.len();
        let mut forces = vec![Vec3::zero(); n];
        for i in 0..n {
            for j in (i + 1)..n {
                let f = gravitational_force(&self.particles[i], &self.particles[j]);
                forces[i] = &forces[i] + &f;
                forces[j] = &forces[j] + &(-&f);
            }
        }
        for (fi, pi) in forces.iter().zip(self.particles.iter_mut()) {
            let accel = fi.scale(1.0 / pi.mass);
            pi.velocity = &pi.velocity + &accel.scale(self.dt);
            pi.position = &pi.position + &pi.velocity.scale(self.dt);
        }
    }

    pub fn step_em(&mut self, e_field: &Vec3, b_field: &Vec3) {
        for p in &mut self.particles {
            let f = lorentz_force(p.charge, &p.velocity, e_field, b_field);
            let accel = f.scale(1.0 / p.mass);
            p.velocity = &p.velocity + &accel.scale(self.dt);
            p.position = &p.position + &p.velocity.scale(self.dt);
        }
    }

    pub fn total_kinetic_energy(&self) -> f64 {
        self.particles.iter().map(|p| p.kinetic_energy()).sum()
    }

    pub fn total_momentum(&self) -> Vec3 {
        self.particles
            .iter()
            .fold(Vec3::zero(), |acc, p| &acc + &p.momentum())
    }

    pub fn center_of_mass(&self) -> Vec3 {
        let total_mass: f64 = self.particles.iter().map(|p| p.mass).sum();
        if total_mass < 1e-30 {
            return Vec3::zero();
        }
        let weighted = self
            .particles
            .iter()
            .fold(Vec3::zero(), |acc, p| &acc + &p.position.scale(p.mass));
        weighted.scale(1.0 / total_mass)
    }
}
