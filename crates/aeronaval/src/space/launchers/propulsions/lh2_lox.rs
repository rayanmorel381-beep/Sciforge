#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propellant {
    LiquidH2Ox,
}

#[derive(Debug, Clone)]
pub struct Stage {
    pub thrust_kn: f64,
    pub isp_s: f64,
    pub propellant: Propellant,
    pub burn_time_s: f64,
}

#[derive(Debug, Clone)]
pub struct Launcher {
    pub stages: Vec<Stage>,
    pub payload_leo_kg: f64,
    pub payload_gto_kg: f64,
}

pub fn heavy() -> Launcher {
    Launcher {
        stages: vec![
            Stage { thrust_kn: 11_000.0, isp_s: 431.0, propellant: Propellant::LiquidH2Ox, burn_time_s: 540.0 },
            Stage { thrust_kn: 1_390.0, isp_s: 446.0, propellant: Propellant::LiquidH2Ox, burn_time_s: 480.0 },
            Stage { thrust_kn: 158.0, isp_s: 446.0, propellant: Propellant::LiquidH2Ox, burn_time_s: 1_100.0 },
        ],
        payload_leo_kg: 64_000.0,
        payload_gto_kg: 21_000.0,
    }
}

pub fn all() -> Vec<Launcher> {
    vec![heavy()]
}
