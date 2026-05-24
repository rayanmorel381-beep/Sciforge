#[derive(Debug, Clone, Copy)]
pub struct WorkFunction {
    pub formula: &'static str,
    pub phi_ev: f64,
}

pub const TABLE: &[WorkFunction] = &[
    WorkFunction { formula: "Cs", phi_ev: 1.95 },
    WorkFunction { formula: "K",  phi_ev: 2.30 },
    WorkFunction { formula: "Na", phi_ev: 2.36 },
    WorkFunction { formula: "Li", phi_ev: 2.93 },
    WorkFunction { formula: "Ba", phi_ev: 2.52 },
    WorkFunction { formula: "Ca", phi_ev: 2.87 },
    WorkFunction { formula: "Mg", phi_ev: 3.66 },
    WorkFunction { formula: "Al", phi_ev: 4.28 },
    WorkFunction { formula: "Ti", phi_ev: 4.33 },
    WorkFunction { formula: "Zn", phi_ev: 4.33 },
    WorkFunction { formula: "Ag", phi_ev: 4.26 },
    WorkFunction { formula: "Cu", phi_ev: 4.65 },
    WorkFunction { formula: "Fe", phi_ev: 4.50 },
    WorkFunction { formula: "Ni", phi_ev: 5.15 },
    WorkFunction { formula: "Co", phi_ev: 5.00 },
    WorkFunction { formula: "Cr", phi_ev: 4.50 },
    WorkFunction { formula: "Mn", phi_ev: 4.10 },
    WorkFunction { formula: "Mo", phi_ev: 4.60 },
    WorkFunction { formula: "W",  phi_ev: 4.55 },
    WorkFunction { formula: "Pt", phi_ev: 5.65 },
    WorkFunction { formula: "Au", phi_ev: 5.10 },
    WorkFunction { formula: "Pb", phi_ev: 4.25 },
    WorkFunction { formula: "Sn", phi_ev: 4.42 },
    WorkFunction { formula: "Hg", phi_ev: 4.49 },
    WorkFunction { formula: "U",  phi_ev: 3.63 },
    WorkFunction { formula: "C",  phi_ev: 5.00 },
    WorkFunction { formula: "Si", phi_ev: 4.85 },
    WorkFunction { formula: "Ge", phi_ev: 5.00 },
];

pub fn by_formula(formula: &str) -> Option<&'static WorkFunction> {
    TABLE.iter().find(|w| w.formula == formula)
}
