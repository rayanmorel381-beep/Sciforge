#[derive(Debug, Clone, Copy)]
pub struct IsothermalCompressibility {
    pub formula: &'static str,
    pub kappa_t_per_pa: f64,
    pub temperature_k: f64,
    pub phase: &'static str,
}

pub const TABLE: &[IsothermalCompressibility] = &[
    IsothermalCompressibility { formula: "H2O",    kappa_t_per_pa: 4.59e-10, temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "Hg",     kappa_t_per_pa: 4.04e-11, temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "CH3OH",  kappa_t_per_pa: 1.22e-9,  temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "C2H5OH", kappa_t_per_pa: 1.11e-9,  temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "C6H6",   kappa_t_per_pa: 9.44e-10, temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "C7H8",   kappa_t_per_pa: 9.10e-10, temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "CCl4",   kappa_t_per_pa: 1.07e-9,  temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "CHCl3",  kappa_t_per_pa: 1.00e-9,  temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "C3H6O",  kappa_t_per_pa: 1.27e-9,  temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "C6H14",  kappa_t_per_pa: 1.67e-9,  temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "C8H18",  kappa_t_per_pa: 1.30e-9,  temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "C3H8O3", kappa_t_per_pa: 2.21e-10, temperature_k: 298.15, phase: "liquid" },
    IsothermalCompressibility { formula: "NH3",    kappa_t_per_pa: 2.34e-9,  temperature_k: 240.0,  phase: "liquid" },
    IsothermalCompressibility { formula: "Fe",     kappa_t_per_pa: 5.86e-12, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "Cu",     kappa_t_per_pa: 7.20e-12, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "Al",     kappa_t_per_pa: 1.32e-11, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "Au",     kappa_t_per_pa: 5.90e-12, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "Ag",     kappa_t_per_pa: 9.87e-12, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "Ni",     kappa_t_per_pa: 5.32e-12, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "W",      kappa_t_per_pa: 3.20e-12, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "Pb",     kappa_t_per_pa: 2.40e-11, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "Si",     kappa_t_per_pa: 1.02e-11, temperature_k: 298.15, phase: "solid" },
    IsothermalCompressibility { formula: "C",      kappa_t_per_pa: 1.80e-12, temperature_k: 298.15, phase: "solid" },
];

pub fn by_formula(formula: &str) -> Option<&'static IsothermalCompressibility> {
    TABLE.iter().find(|c| c.formula == formula)
}
