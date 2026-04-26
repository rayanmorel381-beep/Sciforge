pub struct HillMuscle {
    pub f_max: f64,
    pub l_opt: f64,
    pub v_max: f64,
    pub a_hill: f64,
    pub b_hill: f64,
    pub k_se: f64,
    pub activation: f64,
}

impl HillMuscle {
    pub fn new(f_max: f64, l_opt: f64, v_max: f64) -> Self {
        let a = 0.25 * f_max;
        let b = a * v_max / f_max;
        Self {
            f_max,
            l_opt,
            v_max,
            a_hill: a,
            b_hill: b,
            k_se: 50.0,
            activation: 1.0,
        }
    }

    pub fn force_length(&self, length: f64) -> f64 {
        let ratio = length / self.l_opt;
        let x = (ratio - 1.0) / 0.5;
        self.f_max * (-x * x).exp()
    }

    pub fn force_velocity(&self, velocity: f64) -> f64 {
        if velocity <= 0.0 {
            (self.f_max * self.b_hill - self.a_hill * velocity) / (self.b_hill - velocity)
        } else {
            let f_ecc = 1.5 * self.f_max;
            self.f_max + (f_ecc - self.f_max) * (1.0 - (-velocity / (0.1 * self.v_max)).exp())
        }
    }

    pub fn total_force(&self, length: f64, velocity: f64) -> f64 {
        self.activation * self.force_length(length) * self.force_velocity(velocity) / self.f_max
    }
}

pub fn cross_bridge_huxley(x: f64, f_rate: f64, g_rate: f64, dt: f64, n: f64) -> f64 {
    let dn = f_rate * (1.0 - n) * x.max(0.0) - g_rate * n;
    (n + dn * dt).clamp(0.0, 1.0)
}

pub fn pennation_angle_force(f_tendon: f64, angle_rad: f64) -> f64 {
    f_tendon * angle_rad.cos()
}

pub fn joint_torque(force: f64, moment_arm: f64) -> f64 {
    force * moment_arm
}

pub fn angular_impulse(torque: f64, dt: f64) -> f64 {
    torque * dt
}

pub fn muscle_power(force: f64, velocity: f64) -> f64 {
    force * velocity
}

pub fn work(force: f64, displacement: f64) -> f64 {
    force * displacement
}

pub fn tendon_force(stiffness: f64, strain: f64) -> f64 {
    if strain <= 0.0 {
        return 0.0;
    }
    stiffness * strain
}

pub fn excitation_contraction_coupling(calcium: f64, k_half: f64, n: f64) -> f64 {
    let cn = calcium.powf(n);
    cn / (k_half.powf(n) + cn)
}

pub fn fatigue_model(force_max: f64, time: f64, fatigue_rate: f64) -> f64 {
    force_max * (-fatigue_rate * time).exp()
}

pub fn muscle_stiffness(force: f64, length: f64, l_opt: f64) -> f64 {
    let ratio = length / l_opt;
    force * (2.0 * (ratio - 1.0) / (0.5 * 0.5)) / l_opt
}

pub fn isometric_twitch(f_max: f64, t: f64, tp: f64) -> f64 {
    f_max * (t / tp) * (-(t / tp - 1.0).powi(2)).exp()
}

pub fn tetanus_fusion(f_twitch: f64, frequency: f64, fusion_freq: f64) -> f64 {
    f_twitch * (1.0 + (frequency / fusion_freq).min(3.0))
}

pub fn muscle_metabolic_rate(force: f64, velocity: f64, activation: f64, basal: f64) -> f64 {
    basal + activation * force.abs() + 0.25 * force * velocity.abs()
}

pub fn fiber_type_recruitment(
    excitation: f64,
    threshold_slow: f64,
    threshold_fast: f64,
) -> (f64, f64) {
    let slow = (excitation / threshold_slow).clamp(0.0, 1.0);
    let fast = ((excitation - threshold_fast) / threshold_fast).clamp(0.0, 1.0);
    (slow, fast)
}

pub fn sarcomere_force_length(sl: f64) -> f64 {
    if !(1.27..=3.94).contains(&sl) {
        return 0.0;
    }
    if sl <= 1.7 {
        return (sl - 1.27) / (1.7 - 1.27);
    }
    if sl <= 2.2 {
        return 1.0;
    }
    1.0 - (sl - 2.2) / (3.94 - 2.2)
}
