#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propellant {
    LiquidKeroseneOx,
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

pub fn light() -> Launcher {
    Launcher {
        stages: vec![
            Stage { thrust_kn: 660.0, isp_s: 282.0, propellant: Propellant::LiquidKeroseneOx, burn_time_s: 162.0 },
            Stage { thrust_kn: 95.0, isp_s: 327.0, propellant: Propellant::LiquidKeroseneOx, burn_time_s: 370.0 },
        ],
        payload_leo_kg: 6_000.0,
        payload_gto_kg: 2_500.0,
    }
}

pub fn medium() -> Launcher {
    Launcher {
        stages: vec![
            Stage { thrust_kn: 7_607.0, isp_s: 282.0, propellant: Propellant::LiquidKeroseneOx, burn_time_s: 162.0 },
            Stage { thrust_kn: 934.0, isp_s: 348.0, propellant: Propellant::LiquidKeroseneOx, burn_time_s: 397.0 },
        ],
        payload_leo_kg: 22_800.0,
        payload_gto_kg: 8_300.0,
    }
}

pub fn all() -> Vec<Launcher> {
    vec![light(), medium()]
}
