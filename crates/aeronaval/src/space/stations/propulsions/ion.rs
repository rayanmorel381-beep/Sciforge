#[derive(Debug, Clone)]
pub struct OrbitalThruster {
    pub thrust_n: f64,
    pub isp_s: f64,
    pub count: u8,
}

pub fn ion_maintenance() -> OrbitalThruster {
    OrbitalThruster {
        thrust_n: 0.08,
        isp_s: 4_500.0,
        count: 4,
    }
}

pub fn all() -> Vec<OrbitalThruster> {
    vec![ion_maintenance()]
}
