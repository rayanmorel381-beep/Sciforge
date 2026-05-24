#[derive(Debug, Clone, Copy)]
pub struct Dbtt {
    pub formula: &'static str,
    pub transition_k: f64,
}

pub const TABLE: &[Dbtt] = &[
    Dbtt { formula: "Fe",          transition_k: 220.0 },
    Dbtt { formula: "AISI_1020",   transition_k: 250.0 },
    Dbtt { formula: "AISI_1045",   transition_k: 270.0 },
    Dbtt { formula: "AISI_4140",   transition_k: 230.0 },
    Dbtt { formula: "AISI_A36",    transition_k: 240.0 },
    Dbtt { formula: "AISI_A516",   transition_k: 230.0 },
    Dbtt { formula: "Cr_steel",    transition_k: 180.0 },
    Dbtt { formula: "Ni_steel_3p5",transition_k: 170.0 },
    Dbtt { formula: "Ni_steel_9",  transition_k: 100.0 },
    Dbtt { formula: "W",           transition_k: 670.0 },
    Dbtt { formula: "Mo",          transition_k: 470.0 },
    Dbtt { formula: "Cr",          transition_k: 470.0 },
    Dbtt { formula: "Ta",          transition_k: 30.0 },
    Dbtt { formula: "Nb",          transition_k: 80.0 },
];

pub fn by_formula(formula: &str) -> Option<&'static Dbtt> {
    TABLE.iter().find(|d| d.formula == formula)
}

pub fn is_brittle_at(formula: &str, t_k: f64) -> Option<bool> {
    by_formula(formula).map(|d| t_k < d.transition_k)
}
