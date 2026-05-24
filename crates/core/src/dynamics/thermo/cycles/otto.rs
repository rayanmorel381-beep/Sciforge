use crate::moleculars::Gas;
use sciforge_hub::prelude::physics::thermodynamics as sf_thermo;

pub struct OttoCycle<'a> {
    pub gas: &'a Gas,
    pub compression_ratio: f64,
    pub t1_k: f64,
    pub p1_pa: f64,
    pub q_in_j_kg: f64,
}

impl<'a> OttoCycle<'a> {
    pub fn new(
        gas: &'a Gas,
        compression_ratio: f64,
        t1_k: f64,
        p1_pa: f64,
        q_in_j_kg: f64,
    ) -> Self {
        Self { gas, compression_ratio, t1_k, p1_pa, q_in_j_kg }
    }

    pub fn efficiency(&self) -> f64 {
        sf_thermo::otto_efficiency(self.compression_ratio, self.gas.gamma())
    }

    pub fn t2_k(&self) -> f64 {
        self.t1_k * self.compression_ratio.powf(self.gas.gamma() - 1.0)
    }

    pub fn t3_k(&self) -> f64 {
        self.t2_k() + self.q_in_j_kg / self.gas.cv_j_kgk_ref
    }

    pub fn t4_k(&self) -> f64 {
        self.t3_k() / self.compression_ratio.powf(self.gas.gamma() - 1.0)
    }

    pub fn p2_pa(&self) -> f64 {
        self.p1_pa * self.compression_ratio.powf(self.gas.gamma())
    }

    pub fn net_work_j_kg(&self) -> f64 {
        self.q_in_j_kg * self.efficiency()
    }

    pub fn heat_rejected_j_kg(&self) -> f64 {
        self.gas.cv_j_kgk_ref * (self.t4_k() - self.t1_k)
    }

    pub fn mep_pa(&self) -> f64 {
        let r = self.gas.specific_gas_constant_j_kgk();
        let v1 = r * self.t1_k / self.p1_pa;
        self.net_work_j_kg() / (v1 * (1.0 - 1.0 / self.compression_ratio))
    }
}
