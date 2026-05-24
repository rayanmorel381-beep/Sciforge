#[derive(Debug, Clone, Copy)]
pub struct TransmissionWindow {
    pub formula: &'static str,
    pub uv_cutoff_nm: f64,
    pub ir_cutoff_nm: f64,
    pub typical_transmittance: f64,
}

pub const TABLE: &[TransmissionWindow] = &[
    TransmissionWindow { formula: "BK7",     uv_cutoff_nm: 350.0,  ir_cutoff_nm: 2500.0,  typical_transmittance: 0.92 },
    TransmissionWindow { formula: "SiO2",    uv_cutoff_nm: 200.0,  ir_cutoff_nm: 3500.0,  typical_transmittance: 0.93 },
    TransmissionWindow { formula: "CaF2",    uv_cutoff_nm: 130.0,  ir_cutoff_nm: 11000.0, typical_transmittance: 0.95 },
    TransmissionWindow { formula: "MgF2",    uv_cutoff_nm: 110.0,  ir_cutoff_nm: 7500.0,  typical_transmittance: 0.94 },
    TransmissionWindow { formula: "ZnSe",    uv_cutoff_nm: 600.0,  ir_cutoff_nm: 21000.0, typical_transmittance: 0.71 },
    TransmissionWindow { formula: "Ge",      uv_cutoff_nm: 1800.0, ir_cutoff_nm: 23000.0, typical_transmittance: 0.47 },
    TransmissionWindow { formula: "Si",      uv_cutoff_nm: 1100.0, ir_cutoff_nm: 9000.0,  typical_transmittance: 0.54 },
    TransmissionWindow { formula: "Al2O3",   uv_cutoff_nm: 170.0,  ir_cutoff_nm: 5500.0,  typical_transmittance: 0.85 },
    TransmissionWindow { formula: "KBr",     uv_cutoff_nm: 200.0,  ir_cutoff_nm: 28000.0, typical_transmittance: 0.92 },
    TransmissionWindow { formula: "NaCl",    uv_cutoff_nm: 200.0,  ir_cutoff_nm: 17000.0, typical_transmittance: 0.92 },
    TransmissionWindow { formula: "KCl",     uv_cutoff_nm: 180.0,  ir_cutoff_nm: 25000.0, typical_transmittance: 0.92 },
    TransmissionWindow { formula: "ZnS",     uv_cutoff_nm: 400.0,  ir_cutoff_nm: 14000.0, typical_transmittance: 0.73 },
    TransmissionWindow { formula: "BaF2",    uv_cutoff_nm: 150.0,  ir_cutoff_nm: 13000.0, typical_transmittance: 0.93 },
];

pub fn by_formula(formula: &str) -> Option<&'static TransmissionWindow> {
    TABLE.iter().find(|t| t.formula == formula)
}
