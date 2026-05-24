#[derive(Debug, Clone, Copy)]
pub struct Damping {
    pub formula: &'static str,
    pub loss_factor_eta: f64,
    pub q_factor: f64,
}

pub const TABLE: &[Damping] = &[
    Damping { formula: "Fe",          loss_factor_eta: 1.0e-4, q_factor: 1.0e4 },
    Damping { formula: "AISI_1020",   loss_factor_eta: 6.0e-4, q_factor: 1666.0 },
    Damping { formula: "AISI_304",    loss_factor_eta: 8.0e-4, q_factor: 1250.0 },
    Damping { formula: "Al",          loss_factor_eta: 1.0e-4, q_factor: 1.0e4 },
    Damping { formula: "Al_6061_T6",  loss_factor_eta: 5.0e-4, q_factor: 2000.0 },
    Damping { formula: "Cu",          loss_factor_eta: 2.0e-3, q_factor: 500.0 },
    Damping { formula: "Cu_brass",    loss_factor_eta: 1.0e-3, q_factor: 1000.0 },
    Damping { formula: "Pb",          loss_factor_eta: 1.5e-2, q_factor: 67.0 },
    Damping { formula: "Mg",          loss_factor_eta: 1.0e-2, q_factor: 100.0 },
    Damping { formula: "Ti_6Al_4V",   loss_factor_eta: 3.0e-4, q_factor: 3333.0 },
    Damping { formula: "concrete",    loss_factor_eta: 4.0e-2, q_factor: 25.0 },
    Damping { formula: "wood_pine",   loss_factor_eta: 1.0e-2, q_factor: 100.0 },
    Damping { formula: "PE",          loss_factor_eta: 1.0e-1, q_factor: 10.0 },
    Damping { formula: "PMMA",        loss_factor_eta: 4.0e-2, q_factor: 25.0 },
    Damping { formula: "PVC",         loss_factor_eta: 1.5e-1, q_factor: 6.7 },
    Damping { formula: "rubber",      loss_factor_eta: 5.0e-1, q_factor: 2.0 },
    Damping { formula: "glass_soda",  loss_factor_eta: 1.0e-3, q_factor: 1000.0 },
    Damping { formula: "Al2O3",       loss_factor_eta: 1.0e-4, q_factor: 1.0e4 },
    Damping { formula: "Si",          loss_factor_eta: 1.0e-5, q_factor: 1.0e5 },
    Damping { formula: "SiO2",        loss_factor_eta: 1.0e-5, q_factor: 1.0e5 },
];

pub fn by_formula(formula: &str) -> Option<&'static Damping> {
    TABLE.iter().find(|d| d.formula == formula)
}
