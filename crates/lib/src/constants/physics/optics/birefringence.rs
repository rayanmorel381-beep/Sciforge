#[derive(Debug, Clone, Copy)]
pub struct Birefringence {
    pub formula: &'static str,
    pub n_ordinary: f64,
    pub n_extraordinary: f64,
    pub kind: &'static str,
}

pub const TABLE: &[Birefringence] = &[
    Birefringence { formula: "CaCO3",  n_ordinary: 1.658, n_extraordinary: 1.486, kind: "negative" },
    Birefringence { formula: "SiO2-a", n_ordinary: 1.544, n_extraordinary: 1.553, kind: "positive" },
    Birefringence { formula: "MgF2",   n_ordinary: 1.378, n_extraordinary: 1.390, kind: "positive" },
    Birefringence { formula: "KDP",    n_ordinary: 1.5095,n_extraordinary: 1.4684,kind: "negative" },
    Birefringence { formula: "BBO",    n_ordinary: 1.6551,n_extraordinary: 1.5425,kind: "negative" },
    Birefringence { formula: "LiNbO3", n_ordinary: 2.286, n_extraordinary: 2.200, kind: "negative" },
    Birefringence { formula: "TiO2",   n_ordinary: 2.616, n_extraordinary: 2.903, kind: "positive" },
    Birefringence { formula: "YVO4",   n_ordinary: 1.9929,n_extraordinary: 2.2154,kind: "positive" },
    Birefringence { formula: "Al2O3",  n_ordinary: 1.768, n_extraordinary: 1.760, kind: "negative" },
];

pub fn by_formula(formula: &str) -> Option<&'static Birefringence> {
    TABLE.iter().find(|b| b.formula == formula)
}
