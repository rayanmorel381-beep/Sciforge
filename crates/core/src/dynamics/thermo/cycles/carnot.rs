use sciforge_hub::prelude::physics::thermodynamics as sf_thermo;

pub struct CarnotCycle {
    pub t_hot_k: f64,
    pub t_cold_k: f64,
}

impl CarnotCycle {
    pub fn new(t_hot_k: f64, t_cold_k: f64) -> Self {
        Self { t_hot_k, t_cold_k }
    }

    pub fn efficiency(&self) -> f64 {
        sf_thermo::carnot_efficiency(self.t_hot_k, self.t_cold_k)
    }

    pub fn cop_refrigerator(&self) -> f64 {
        sf_thermo::carnot_cop_cooling(self.t_hot_k, self.t_cold_k)
    }

    pub fn cop_heat_pump(&self) -> f64 {
        sf_thermo::carnot_cop_heating(self.t_hot_k, self.t_cold_k)
    }

    pub fn work_out_j(&self, q_hot_j: f64) -> f64 {
        q_hot_j * self.efficiency()
    }

    pub fn heat_rejected_j(&self, q_hot_j: f64) -> f64 {
        q_hot_j * self.t_cold_k / self.t_hot_k
    }

    pub fn q_hot_from_work_j(&self, work_j: f64) -> f64 {
        work_j / self.efficiency()
    }
}
