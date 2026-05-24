#[derive(Debug, Clone, Copy)]
pub struct ElectricalResistivity {
    pub formula: &'static str,
    pub rho_ohm_m: f64,
    pub temperature_k: f64,
    pub temperature_coeff_per_k: f64,
}

pub const TABLE: &[ElectricalResistivity] = &[
    ElectricalResistivity { formula: "Ag", rho_ohm_m: 1.587e-8, temperature_k: 293.15, temperature_coeff_per_k: 3.80e-3 },
    ElectricalResistivity { formula: "Cu", rho_ohm_m: 1.678e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.04e-3 },
    ElectricalResistivity { formula: "Au", rho_ohm_m: 2.214e-8, temperature_k: 293.15, temperature_coeff_per_k: 3.40e-3 },
    ElectricalResistivity { formula: "Al", rho_ohm_m: 2.650e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.29e-3 },
    ElectricalResistivity { formula: "W",  rho_ohm_m: 5.600e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.50e-3 },
    ElectricalResistivity { formula: "Mg", rho_ohm_m: 4.390e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.20e-3 },
    ElectricalResistivity { formula: "Zn", rho_ohm_m: 5.900e-8, temperature_k: 293.15, temperature_coeff_per_k: 3.70e-3 },
    ElectricalResistivity { formula: "Ni", rho_ohm_m: 6.990e-8, temperature_k: 293.15, temperature_coeff_per_k: 6.00e-3 },
    ElectricalResistivity { formula: "Fe", rho_ohm_m: 9.710e-8, temperature_k: 293.15, temperature_coeff_per_k: 5.00e-3 },
    ElectricalResistivity { formula: "Pt", rho_ohm_m: 10.60e-8, temperature_k: 293.15, temperature_coeff_per_k: 3.93e-3 },
    ElectricalResistivity { formula: "Sn", rho_ohm_m: 11.50e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.50e-3 },
    ElectricalResistivity { formula: "Pb", rho_ohm_m: 22.00e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.20e-3 },
    ElectricalResistivity { formula: "Ti", rho_ohm_m: 42.00e-8, temperature_k: 293.15, temperature_coeff_per_k: 3.80e-3 },
    ElectricalResistivity { formula: "Hg", rho_ohm_m: 9.610e-7, temperature_k: 293.15, temperature_coeff_per_k: 9.00e-4 },
    ElectricalResistivity { formula: "Mn", rho_ohm_m: 1.440e-6, temperature_k: 293.15, temperature_coeff_per_k: 1.00e-5 },
    ElectricalResistivity { formula: "Cr", rho_ohm_m: 1.250e-7, temperature_k: 293.15, temperature_coeff_per_k: 5.90e-3 },
    ElectricalResistivity { formula: "Co", rho_ohm_m: 6.240e-8, temperature_k: 293.15, temperature_coeff_per_k: 6.00e-3 },
    ElectricalResistivity { formula: "Mo", rho_ohm_m: 5.340e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.37e-3 },
    ElectricalResistivity { formula: "Li", rho_ohm_m: 9.280e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.60e-3 },
    ElectricalResistivity { formula: "Na", rho_ohm_m: 4.770e-8, temperature_k: 293.15, temperature_coeff_per_k: 5.50e-3 },
    ElectricalResistivity { formula: "K",  rho_ohm_m: 7.200e-8, temperature_k: 293.15, temperature_coeff_per_k: 5.80e-3 },
    ElectricalResistivity { formula: "Ca", rho_ohm_m: 3.360e-8, temperature_k: 293.15, temperature_coeff_per_k: 4.10e-3 },
    ElectricalResistivity { formula: "U",  rho_ohm_m: 2.800e-7, temperature_k: 293.15, temperature_coeff_per_k: 2.10e-3 },

    ElectricalResistivity { formula: "C",     rho_ohm_m: 1.00e-5, temperature_k: 293.15, temperature_coeff_per_k: -5.0e-4 },
    ElectricalResistivity { formula: "Si",    rho_ohm_m: 2.30e3,  temperature_k: 293.15, temperature_coeff_per_k: -7.5e-2 },
    ElectricalResistivity { formula: "Ge",    rho_ohm_m: 4.60e-1, temperature_k: 293.15, temperature_coeff_per_k: -4.8e-2 },
    ElectricalResistivity { formula: "SiO2",  rho_ohm_m: 1.00e16, temperature_k: 293.15, temperature_coeff_per_k:  0.0    },
    ElectricalResistivity { formula: "Al2O3", rho_ohm_m: 1.00e14, temperature_k: 293.15, temperature_coeff_per_k:  0.0    },
    ElectricalResistivity { formula: "H2O",   rho_ohm_m: 1.82e5,  temperature_k: 298.15, temperature_coeff_per_k:  0.0    },
    ElectricalResistivity { formula: "Hg",    rho_ohm_m: 9.61e-7, temperature_k: 234.32, temperature_coeff_per_k:  9.0e-4 },
];

impl ElectricalResistivity {
    pub fn rho_at(&self, temperature_k: f64) -> f64 {
        self.rho_ohm_m * (1.0 + self.temperature_coeff_per_k * (temperature_k - self.temperature_k))
    }

    pub fn conductivity_s_per_m(&self) -> f64 {
        1.0 / self.rho_ohm_m
    }
}

pub fn by_formula(formula: &str) -> Option<&'static ElectricalResistivity> {
    TABLE.iter().find(|r| r.formula == formula)
}

pub const CONDUCTOR_MAX_OHM_M: f64 = 1.0e-3;
pub const INSULATOR_MIN_OHM_M: f64 = 1.0e8;
pub const ELECTROLYTE_MIN_OHM_M: f64 = 1.0e-2;
pub const ELECTROLYTE_MAX_OHM_M: f64 = 1.0e6;

pub fn is_conductor(rho_ohm_m: f64) -> bool {
    rho_ohm_m < CONDUCTOR_MAX_OHM_M
}

pub fn is_insulator(rho_ohm_m: f64) -> bool {
    rho_ohm_m > INSULATOR_MIN_OHM_M
}

pub fn is_semiconductor(rho_ohm_m: f64) -> bool {
    (CONDUCTOR_MAX_OHM_M..=INSULATOR_MIN_OHM_M).contains(&rho_ohm_m)
}

pub fn is_electrolyte(rho_ohm_m: f64) -> bool {
    (ELECTROLYTE_MIN_OHM_M..=ELECTROLYTE_MAX_OHM_M).contains(&rho_ohm_m)
}
