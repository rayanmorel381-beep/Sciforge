#[derive(Debug, Clone, Copy)]
pub struct LiquidViscosity {
    pub formula: &'static str,
    pub mu_pa_s: f64,
    pub temperature_k: f64,
}

pub const TABLE: &[LiquidViscosity] = &[
    LiquidViscosity { formula: "H2O",    mu_pa_s: 8.90e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "Hg",     mu_pa_s: 1.526e-3, temperature_k: 298.15 },
    LiquidViscosity { formula: "CH3OH",  mu_pa_s: 5.44e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C2H5OH", mu_pa_s: 1.074e-3, temperature_k: 298.15 },
    LiquidViscosity { formula: "C3H8O3", mu_pa_s: 9.34e-1,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C6H6",   mu_pa_s: 6.04e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C7H8",   mu_pa_s: 5.50e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C8H10",  mu_pa_s: 6.20e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "CHCl3",  mu_pa_s: 5.42e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "CH2Cl2", mu_pa_s: 4.13e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "CCl4",   mu_pa_s: 9.08e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C3H6O",  mu_pa_s: 3.06e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C5H12",  mu_pa_s: 2.24e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C6H14",  mu_pa_s: 2.94e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C7H16",  mu_pa_s: 3.86e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "C8H18",  mu_pa_s: 5.08e-4,  temperature_k: 298.15 },
    LiquidViscosity { formula: "NH3",    mu_pa_s: 1.380e-4, temperature_k: 240.0  },
    LiquidViscosity { formula: "N2",     mu_pa_s: 1.610e-4, temperature_k: 77.0   },
    LiquidViscosity { formula: "O2",     mu_pa_s: 1.950e-4, temperature_k: 90.0   },
];

pub fn by_formula(formula: &str) -> Option<&'static LiquidViscosity> {
    TABLE.iter().find(|v| v.formula == formula)
}
