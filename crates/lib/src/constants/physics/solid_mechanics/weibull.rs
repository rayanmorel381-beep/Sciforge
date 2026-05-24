#[derive(Debug, Clone, Copy)]
pub struct Weibull {
    pub formula: &'static str,
    pub m_modulus: f64,
    pub sigma_0_pa: f64,
    pub volume_ref_m3: f64,
}

pub const TABLE: &[Weibull] = &[
    Weibull { formula: "Al2O3",      m_modulus: 11.0, sigma_0_pa: 3.50e8, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "SiC",        m_modulus: 9.0,  sigma_0_pa: 4.50e8, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "Si3N4",      m_modulus: 15.0, sigma_0_pa: 7.00e8, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "ZrO2",       m_modulus: 12.0, sigma_0_pa: 9.00e8, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "B4C",        m_modulus: 8.0,  sigma_0_pa: 4.00e8, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "WC",         m_modulus: 16.0, sigma_0_pa: 1.50e9, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "MgO",        m_modulus: 10.0, sigma_0_pa: 2.50e8, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "glass_soda", m_modulus: 6.0,  sigma_0_pa: 7.00e7, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "BK7",        m_modulus: 7.0,  sigma_0_pa: 8.00e7, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "SiO2",       m_modulus: 7.0,  sigma_0_pa: 7.50e7, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "C_CFRP",     m_modulus: 25.0, sigma_0_pa: 2.20e9, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "GFRP",       m_modulus: 18.0, sigma_0_pa: 1.20e9, volume_ref_m3: 1.0e-6 },
    Weibull { formula: "concrete",   m_modulus: 12.0, sigma_0_pa: 4.00e7, volume_ref_m3: 1.0e-3 },
];

pub fn by_formula(formula: &str) -> Option<&'static Weibull> {
    TABLE.iter().find(|w| w.formula == formula)
}

pub fn failure_probability(entry: &Weibull, sigma_pa: f64, volume_m3: f64) -> f64 {
    1.0 - (-(volume_m3 / entry.volume_ref_m3) * (sigma_pa / entry.sigma_0_pa).powf(entry.m_modulus)).exp()
}
