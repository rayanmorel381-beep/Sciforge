#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub thrust_n: f64,
    pub isp_s: f64,
    pub delta_v_ms: f64,
}

pub fn cold_gas_rcs() -> PropulsionKit {
    PropulsionKit {
        thrust_n: 1.0,
        isp_s: 65.0,
        delta_v_ms: 50.0,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![cold_gas_rcs()]
}
