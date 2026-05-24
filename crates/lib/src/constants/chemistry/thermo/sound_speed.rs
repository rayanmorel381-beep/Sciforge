#[derive(Debug, Clone, Copy)]
pub struct SoundSpeed {
    pub formula: &'static str,
    pub c_m_per_s: f64,
    pub temperature_k: f64,
    pub phase: &'static str,
}

pub const TABLE: &[SoundSpeed] = &[
    SoundSpeed { formula: "AIR",  c_m_per_s:  343.21, temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "N2",   c_m_per_s:  349.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "O2",   c_m_per_s:  328.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "H2",   c_m_per_s: 1310.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "He",   c_m_per_s: 1007.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "Ne",   c_m_per_s:  461.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "Ar",   c_m_per_s:  323.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "Kr",   c_m_per_s:  220.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "Xe",   c_m_per_s:  178.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "CO2",  c_m_per_s:  267.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "CO",   c_m_per_s:  349.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "CH4",  c_m_per_s:  446.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "NH3",  c_m_per_s:  431.0,  temperature_k: 293.15, phase: "gas" },
    SoundSpeed { formula: "SF6",  c_m_per_s:  136.0,  temperature_k: 293.15, phase: "gas" },

    SoundSpeed { formula: "H2O",    c_m_per_s: 1497.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "Hg",     c_m_per_s: 1450.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "CH3OH",  c_m_per_s: 1103.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "C2H5OH", c_m_per_s: 1162.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "C3H8O3", c_m_per_s: 1923.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "C6H6",   c_m_per_s: 1295.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "C7H8",   c_m_per_s: 1305.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "C3H6O",  c_m_per_s: 1174.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "CCl4",   c_m_per_s:  926.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "CHCl3",  c_m_per_s:  984.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "C6H14",  c_m_per_s: 1083.0, temperature_k: 298.15, phase: "liquid" },
    SoundSpeed { formula: "C8H18",  c_m_per_s: 1192.0, temperature_k: 298.15, phase: "liquid" },

    SoundSpeed { formula: "Fe",    c_m_per_s: 5957.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Al",    c_m_per_s: 6420.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Cu",    c_m_per_s: 4760.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Au",    c_m_per_s: 3240.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Ag",    c_m_per_s: 3650.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Ni",    c_m_per_s: 5630.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Ti",    c_m_per_s: 6070.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "W",     c_m_per_s: 5410.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Pb",    c_m_per_s: 2160.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Mg",    c_m_per_s: 5770.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Si",    c_m_per_s: 8433.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "C",     c_m_per_s: 18350.0,temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "SiO2",  c_m_per_s: 5968.0, temperature_k: 298.15, phase: "solid" },
    SoundSpeed { formula: "Al2O3", c_m_per_s: 10845.0,temperature_k: 298.15, phase: "solid" },
];

pub fn by_formula(formula: &str) -> Option<&'static SoundSpeed> {
    TABLE.iter().find(|s| s.formula == formula)
}

pub fn by_formula_phase(formula: &str, phase: &str) -> Option<&'static SoundSpeed> {
    TABLE.iter().find(|s| s.formula == formula && s.phase == phase)
}
