#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub formula: &'static str,
    pub relative_permittivity: f64,
    pub dielectric_strength_v_per_m: f64,
    pub loss_tangent: f64,
    pub temperature_k: f64,
}

pub const TABLE: &[Dielectric] = &[
    Dielectric { formula: "VAC",   relative_permittivity: 1.0,      dielectric_strength_v_per_m: f64::INFINITY, loss_tangent: 0.0,      temperature_k: 300.0 },
    Dielectric { formula: "AIR",   relative_permittivity: 1.00059,  dielectric_strength_v_per_m: 3.0e6,         loss_tangent: 0.0,      temperature_k: 300.0 },
    Dielectric { formula: "H2O",   relative_permittivity: 80.1,     dielectric_strength_v_per_m: 6.5e7,         loss_tangent: 0.10,     temperature_k: 293.0 },
    Dielectric { formula: "SiO2",  relative_permittivity: 3.9,      dielectric_strength_v_per_m: 1.0e9,         loss_tangent: 0.0001,   temperature_k: 300.0 },
    Dielectric { formula: "Al2O3", relative_permittivity: 9.5,      dielectric_strength_v_per_m: 1.7e7,         loss_tangent: 0.0001,   temperature_k: 300.0 },
    Dielectric { formula: "Si3N4", relative_permittivity: 7.5,      dielectric_strength_v_per_m: 1.0e9,         loss_tangent: 0.001,    temperature_k: 300.0 },
    Dielectric { formula: "HfO2",  relative_permittivity: 25.0,     dielectric_strength_v_per_m: 5.0e8,         loss_tangent: 0.005,    temperature_k: 300.0 },
    Dielectric { formula: "TiO2",  relative_permittivity: 86.0,     dielectric_strength_v_per_m: 6.0e6,         loss_tangent: 0.001,    temperature_k: 300.0 },
    Dielectric { formula: "BaTiO3",relative_permittivity: 1500.0,   dielectric_strength_v_per_m: 5.0e6,         loss_tangent: 0.01,     temperature_k: 300.0 },
    Dielectric { formula: "PE",    relative_permittivity: 2.3,      dielectric_strength_v_per_m: 2.0e7,         loss_tangent: 0.0002,   temperature_k: 300.0 },
    Dielectric { formula: "PTFE",  relative_permittivity: 2.1,      dielectric_strength_v_per_m: 6.0e7,         loss_tangent: 0.0002,   temperature_k: 300.0 },
    Dielectric { formula: "PVC",   relative_permittivity: 3.0,      dielectric_strength_v_per_m: 4.0e7,         loss_tangent: 0.02,     temperature_k: 300.0 },
    Dielectric { formula: "PS",    relative_permittivity: 2.5,      dielectric_strength_v_per_m: 2.0e7,         loss_tangent: 0.0001,   temperature_k: 300.0 },
    Dielectric { formula: "Mica",  relative_permittivity: 6.0,      dielectric_strength_v_per_m: 1.18e8,        loss_tangent: 0.0003,   temperature_k: 300.0 },
    Dielectric { formula: "Si",    relative_permittivity: 11.7,     dielectric_strength_v_per_m: 3.0e7,         loss_tangent: 0.005,    temperature_k: 300.0 },
    Dielectric { formula: "Ge",    relative_permittivity: 16.0,     dielectric_strength_v_per_m: 1.0e7,         loss_tangent: 0.01,     temperature_k: 300.0 },
    Dielectric { formula: "GaAs",  relative_permittivity: 12.9,     dielectric_strength_v_per_m: 4.0e7,         loss_tangent: 0.001,    temperature_k: 300.0 },
    Dielectric { formula: "GaN",   relative_permittivity: 8.9,      dielectric_strength_v_per_m: 3.0e8,         loss_tangent: 0.001,    temperature_k: 300.0 },
    Dielectric { formula: "SiC",   relative_permittivity: 9.7,      dielectric_strength_v_per_m: 3.0e8,         loss_tangent: 0.0005,   temperature_k: 300.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static Dielectric> {
    TABLE.iter().find(|d| d.formula == formula)
}
