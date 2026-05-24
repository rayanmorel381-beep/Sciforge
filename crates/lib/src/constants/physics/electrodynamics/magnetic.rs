#[derive(Debug, Clone, Copy)]
pub struct MagneticProperties {
    pub formula: &'static str,
    pub relative_permeability: f64,
    pub saturation_flux_density_t: f64,
    pub coercivity_a_per_m: f64,
    pub curie_temperature_k: f64,
    pub kind: &'static str,
}

pub const TABLE: &[MagneticProperties] = &[
    MagneticProperties { formula: "VAC",     relative_permeability: 1.0,        saturation_flux_density_t: 0.0,  coercivity_a_per_m: 0.0,    curie_temperature_k: 0.0,    kind: "vacuum" },
    MagneticProperties { formula: "AIR",     relative_permeability: 1.0000004,  saturation_flux_density_t: 0.0,  coercivity_a_per_m: 0.0,    curie_temperature_k: 0.0,    kind: "para" },
    MagneticProperties { formula: "Cu",      relative_permeability: 0.999994,   saturation_flux_density_t: 0.0,  coercivity_a_per_m: 0.0,    curie_temperature_k: 0.0,    kind: "dia" },
    MagneticProperties { formula: "Al",      relative_permeability: 1.000022,   saturation_flux_density_t: 0.0,  coercivity_a_per_m: 0.0,    curie_temperature_k: 0.0,    kind: "para" },
    MagneticProperties { formula: "Ag",      relative_permeability: 0.99998,    saturation_flux_density_t: 0.0,  coercivity_a_per_m: 0.0,    curie_temperature_k: 0.0,    kind: "dia" },
    MagneticProperties { formula: "Au",      relative_permeability: 0.99996,    saturation_flux_density_t: 0.0,  coercivity_a_per_m: 0.0,    curie_temperature_k: 0.0,    kind: "dia" },
    MagneticProperties { formula: "Fe",      relative_permeability: 5000.0,     saturation_flux_density_t: 2.15, coercivity_a_per_m: 80.0,   curie_temperature_k: 1043.0, kind: "ferro" },
    MagneticProperties { formula: "Ni",      relative_permeability: 600.0,      saturation_flux_density_t: 0.61, coercivity_a_per_m: 56.0,   curie_temperature_k: 627.0,  kind: "ferro" },
    MagneticProperties { formula: "Co",      relative_permeability: 250.0,      saturation_flux_density_t: 1.79, coercivity_a_per_m: 800.0,  curie_temperature_k: 1388.0, kind: "ferro" },
    MagneticProperties { formula: "Mu",      relative_permeability: 80000.0,    saturation_flux_density_t: 0.80, coercivity_a_per_m: 4.0,    curie_temperature_k: 723.0,  kind: "ferro" },
    MagneticProperties { formula: "Permalloy", relative_permeability: 100000.0, saturation_flux_density_t: 1.05, coercivity_a_per_m: 4.0,    curie_temperature_k: 733.0,  kind: "ferro" },
    MagneticProperties { formula: "MnZn",    relative_permeability: 1500.0,     saturation_flux_density_t: 0.50, coercivity_a_per_m: 25.0,   curie_temperature_k: 473.0,  kind: "ferri" },
    MagneticProperties { formula: "NiZn",    relative_permeability: 100.0,      saturation_flux_density_t: 0.35, coercivity_a_per_m: 200.0,  curie_temperature_k: 573.0,  kind: "ferri" },
    MagneticProperties { formula: "NdFeB",   relative_permeability: 1.05,       saturation_flux_density_t: 1.40, coercivity_a_per_m: 8.7e5,  curie_temperature_k: 583.0,  kind: "hard" },
    MagneticProperties { formula: "SmCo",    relative_permeability: 1.05,       saturation_flux_density_t: 1.05, coercivity_a_per_m: 7.0e5,  curie_temperature_k: 1073.0, kind: "hard" },
    MagneticProperties { formula: "AlNiCo",  relative_permeability: 4.0,        saturation_flux_density_t: 1.25, coercivity_a_per_m: 4.0e4,  curie_temperature_k: 1133.0, kind: "hard" },
    MagneticProperties { formula: "Gd",      relative_permeability: 50.0,       saturation_flux_density_t: 2.60, coercivity_a_per_m: 80.0,   curie_temperature_k: 292.0,  kind: "ferro" },
];

pub fn by_formula(formula: &str) -> Option<&'static MagneticProperties> {
    TABLE.iter().find(|m| m.formula == formula)
}
