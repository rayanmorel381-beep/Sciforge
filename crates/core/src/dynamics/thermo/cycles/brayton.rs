use crate::moleculars::Gas;

pub struct BraytonCycle<'a> {
    pub gas: &'a Gas,
    pub t1_k: f64,
    pub pressure_ratio: f64,
    pub t3_k: f64,
    pub eta_compressor: f64,
    pub eta_turbine: f64,
}

impl<'a> BraytonCycle<'a> {
    pub fn new(
        gas: &'a Gas,
        t1_k: f64,
        pressure_ratio: f64,
        t3_k: f64,
        eta_compressor: f64,
        eta_turbine: f64,
    ) -> Self {
        Self { gas, t1_k, pressure_ratio, t3_k, eta_compressor, eta_turbine }
    }

    fn exponent(&self) -> f64 {
        (self.gas.gamma() - 1.0) / self.gas.gamma()
    }

    pub fn t2s_k(&self) -> f64 {
        self.t1_k * self.pressure_ratio.powf(self.exponent())
    }

    pub fn t2_k(&self) -> f64 {
        self.t1_k + (self.t2s_k() - self.t1_k) / self.eta_compressor
    }

    pub fn t4s_k(&self) -> f64 {
        self.t3_k / self.pressure_ratio.powf(self.exponent())
    }

    pub fn t4_k(&self) -> f64 {
        self.t3_k - (self.t3_k - self.t4s_k()) * self.eta_turbine
    }

    pub fn work_compressor_j_kg(&self) -> f64 {
        self.gas.cp_j_kgk_ref * (self.t2_k() - self.t1_k)
    }

    pub fn work_turbine_j_kg(&self) -> f64 {
        self.gas.cp_j_kgk_ref * (self.t3_k - self.t4_k())
    }

    pub fn net_work_j_kg(&self) -> f64 {
        self.work_turbine_j_kg() - self.work_compressor_j_kg()
    }

    pub fn q_in_j_kg(&self) -> f64 {
        self.gas.cp_j_kgk_ref * (self.t3_k - self.t2_k())
    }

    pub fn efficiency(&self) -> f64 {
        self.net_work_j_kg() / self.q_in_j_kg()
    }

    pub fn back_work_ratio(&self) -> f64 {
        self.work_compressor_j_kg() / self.work_turbine_j_kg()
    }

    pub fn specific_power_w_kg_s(&self, mass_flow_kg_s: f64) -> f64 {
        self.net_work_j_kg() * mass_flow_kg_s
    }
}
