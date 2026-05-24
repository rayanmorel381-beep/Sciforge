#[derive(Debug, Clone, Copy)]
pub struct ThermoOptic {
    pub formula: &'static str,
    pub dn_dt_per_k: f64,
    pub temperature_k: f64,
}

pub const TABLE: &[ThermoOptic] = &[
    ThermoOptic { formula: "SiO2",   dn_dt_per_k:  1.28e-5, temperature_k: 293.15 },
    ThermoOptic { formula: "BK7",    dn_dt_per_k:  1.86e-6, temperature_k: 293.15 },
    ThermoOptic { formula: "SF11",   dn_dt_per_k:  6.80e-6, temperature_k: 293.15 },
    ThermoOptic { formula: "CaF2",   dn_dt_per_k: -1.05e-5, temperature_k: 293.15 },
    ThermoOptic { formula: "MgF2",   dn_dt_per_k:  1.10e-6, temperature_k: 293.15 },
    ThermoOptic { formula: "Al2O3",  dn_dt_per_k:  1.30e-5, temperature_k: 293.15 },
    ThermoOptic { formula: "Si",     dn_dt_per_k:  1.86e-4, temperature_k: 293.15 },
    ThermoOptic { formula: "Ge",     dn_dt_per_k:  4.04e-4, temperature_k: 293.15 },
    ThermoOptic { formula: "ZnSe",   dn_dt_per_k:  6.10e-5, temperature_k: 293.15 },
    ThermoOptic { formula: "LiNbO3", dn_dt_per_k:  3.90e-5, temperature_k: 293.15 },
    ThermoOptic { formula: "GaAs",   dn_dt_per_k:  3.66e-4, temperature_k: 293.15 },
    ThermoOptic { formula: "H2O",    dn_dt_per_k: -1.00e-4, temperature_k: 293.15 },
];

pub fn by_formula(formula: &str) -> Option<&'static ThermoOptic> {
    TABLE.iter().find(|t| t.formula == formula)
}
