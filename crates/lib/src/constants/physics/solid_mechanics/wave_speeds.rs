#[derive(Debug, Clone, Copy)]
pub struct WaveSpeeds {
    pub formula: &'static str,
    pub p_wave_m_per_s: f64,
    pub s_wave_m_per_s: f64,
    pub rayleigh_m_per_s: f64,
    pub bar_m_per_s: f64,
}

pub const TABLE: &[WaveSpeeds] = &[
    WaveSpeeds { formula: "Fe",          p_wave_m_per_s: 5960.0, s_wave_m_per_s: 3240.0, rayleigh_m_per_s: 2980.0, bar_m_per_s: 5180.0 },
    WaveSpeeds { formula: "AISI_1020",   p_wave_m_per_s: 5890.0, s_wave_m_per_s: 3240.0, rayleigh_m_per_s: 2980.0, bar_m_per_s: 5100.0 },
    WaveSpeeds { formula: "AISI_304",    p_wave_m_per_s: 5790.0, s_wave_m_per_s: 3100.0, rayleigh_m_per_s: 2870.0, bar_m_per_s: 5000.0 },
    WaveSpeeds { formula: "Al",          p_wave_m_per_s: 6420.0, s_wave_m_per_s: 3040.0, rayleigh_m_per_s: 2900.0, bar_m_per_s: 5050.0 },
    WaveSpeeds { formula: "Al_6061_T6",  p_wave_m_per_s: 6320.0, s_wave_m_per_s: 3100.0, rayleigh_m_per_s: 2900.0, bar_m_per_s: 5050.0 },
    WaveSpeeds { formula: "Cu",          p_wave_m_per_s: 4760.0, s_wave_m_per_s: 2325.0, rayleigh_m_per_s: 2150.0, bar_m_per_s: 3700.0 },
    WaveSpeeds { formula: "Au",          p_wave_m_per_s: 3240.0, s_wave_m_per_s: 1200.0, rayleigh_m_per_s: 1100.0, bar_m_per_s: 2030.0 },
    WaveSpeeds { formula: "Ag",          p_wave_m_per_s: 3650.0, s_wave_m_per_s: 1610.0, rayleigh_m_per_s: 1480.0, bar_m_per_s: 2680.0 },
    WaveSpeeds { formula: "Ti",          p_wave_m_per_s: 6070.0, s_wave_m_per_s: 3125.0, rayleigh_m_per_s: 2900.0, bar_m_per_s: 5080.0 },
    WaveSpeeds { formula: "Ti_6Al_4V",   p_wave_m_per_s: 6100.0, s_wave_m_per_s: 3120.0, rayleigh_m_per_s: 2900.0, bar_m_per_s: 5070.0 },
    WaveSpeeds { formula: "Ni",          p_wave_m_per_s: 5630.0, s_wave_m_per_s: 2960.0, rayleigh_m_per_s: 2730.0, bar_m_per_s: 4800.0 },
    WaveSpeeds { formula: "Pb",          p_wave_m_per_s: 1960.0, s_wave_m_per_s: 690.0,  rayleigh_m_per_s: 640.0,  bar_m_per_s: 1190.0 },
    WaveSpeeds { formula: "W",           p_wave_m_per_s: 5200.0, s_wave_m_per_s: 2870.0, rayleigh_m_per_s: 2640.0, bar_m_per_s: 4620.0 },
    WaveSpeeds { formula: "Mg",          p_wave_m_per_s: 5770.0, s_wave_m_per_s: 3050.0, rayleigh_m_per_s: 2810.0, bar_m_per_s: 4940.0 },
    WaveSpeeds { formula: "Si",          p_wave_m_per_s: 8430.0, s_wave_m_per_s: 5840.0, rayleigh_m_per_s: 5020.0, bar_m_per_s: 7700.0 },
    WaveSpeeds { formula: "diamond",     p_wave_m_per_s: 17500.0,s_wave_m_per_s: 12300.0,rayleigh_m_per_s: 11200.0,bar_m_per_s: 17300.0 },
    WaveSpeeds { formula: "Al2O3",       p_wave_m_per_s: 10520.0,s_wave_m_per_s: 6160.0, rayleigh_m_per_s: 5500.0, bar_m_per_s: 9670.0 },
    WaveSpeeds { formula: "SiC",         p_wave_m_per_s: 11930.0,s_wave_m_per_s: 7340.0, rayleigh_m_per_s: 6620.0, bar_m_per_s: 11290.0 },
    WaveSpeeds { formula: "glass_soda",  p_wave_m_per_s: 5660.0, s_wave_m_per_s: 3420.0, rayleigh_m_per_s: 3140.0, bar_m_per_s: 5290.0 },
    WaveSpeeds { formula: "concrete",    p_wave_m_per_s: 3700.0, s_wave_m_per_s: 2200.0, rayleigh_m_per_s: 2030.0, bar_m_per_s: 3500.0 },
    WaveSpeeds { formula: "wood_pine",   p_wave_m_per_s: 4200.0, s_wave_m_per_s: 1300.0, rayleigh_m_per_s: 1200.0, bar_m_per_s: 4200.0 },
    WaveSpeeds { formula: "PMMA",        p_wave_m_per_s: 2700.0, s_wave_m_per_s: 1300.0, rayleigh_m_per_s: 1200.0, bar_m_per_s: 2200.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static WaveSpeeds> {
    TABLE.iter().find(|w| w.formula == formula)
}
