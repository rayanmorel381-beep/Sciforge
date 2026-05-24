#[derive(Debug, Clone)]
pub struct PropulsionKit {
    pub thrust_n: f64,
    pub isp_s: f64,
    pub delta_v_ms: f64,
}

pub fn ion_drive() -> PropulsionKit {
    PropulsionKit {
        thrust_n: 0.09,
        isp_s: 3_100.0,
        delta_v_ms: 10_000.0,
    }
}

pub fn all() -> Vec<PropulsionKit> {
    vec![ion_drive()]
}
