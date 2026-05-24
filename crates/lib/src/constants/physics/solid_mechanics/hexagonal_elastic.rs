#[derive(Debug, Clone, Copy)]
pub struct HexagonalElastic {
    pub formula: &'static str,
    pub c11_pa: f64,
    pub c12_pa: f64,
    pub c13_pa: f64,
    pub c33_pa: f64,
    pub c44_pa: f64,
}

pub const TABLE: &[HexagonalElastic] = &[
    HexagonalElastic { formula: "Mg",        c11_pa: 5.93e10, c12_pa: 2.57e10, c13_pa: 2.14e10, c33_pa: 6.15e10, c44_pa: 1.64e10 },
    HexagonalElastic { formula: "Ti",        c11_pa: 1.624e11,c12_pa: 9.20e10, c13_pa: 6.90e10, c33_pa: 1.807e11,c44_pa: 4.67e10 },
    HexagonalElastic { formula: "Zn",        c11_pa: 1.610e11,c12_pa: 3.42e10, c13_pa: 5.01e10, c33_pa: 6.10e10, c44_pa: 3.83e10 },
    HexagonalElastic { formula: "Cd",        c11_pa: 1.146e11,c12_pa: 3.95e10, c13_pa: 3.99e10, c33_pa: 5.08e10, c44_pa: 1.99e10 },
    HexagonalElastic { formula: "Be",        c11_pa: 2.923e11,c12_pa: 2.67e10, c13_pa: 1.40e10, c33_pa: 3.364e11,c44_pa: 1.625e11 },
    HexagonalElastic { formula: "Co",        c11_pa: 3.071e11,c12_pa: 1.650e11,c13_pa: 1.027e11,c33_pa: 3.581e11,c44_pa: 7.55e10 },
    HexagonalElastic { formula: "Zr",        c11_pa: 1.434e11,c12_pa: 7.28e10, c13_pa: 6.53e10, c33_pa: 1.648e11,c44_pa: 3.20e10 },
    HexagonalElastic { formula: "SiO2-a",    c11_pa: 8.68e10, c12_pa: 7.04e9,  c13_pa: 1.19e10, c33_pa: 1.058e11,c44_pa: 5.82e10 },
    HexagonalElastic { formula: "Al2O3",     c11_pa: 4.97e11, c12_pa: 1.63e11, c13_pa: 1.11e11, c33_pa: 4.99e11, c44_pa: 1.47e11 },
    HexagonalElastic { formula: "ZnO",       c11_pa: 2.097e11,c12_pa: 1.211e11,c13_pa: 1.051e11,c33_pa: 2.109e11,c44_pa: 4.247e10 },
    HexagonalElastic { formula: "GaN",       c11_pa: 3.90e11, c12_pa: 1.45e11, c13_pa: 1.06e11, c33_pa: 3.98e11, c44_pa: 1.05e11 },
    HexagonalElastic { formula: "AlN",       c11_pa: 4.10e11, c12_pa: 1.40e11, c13_pa: 1.00e11, c33_pa: 3.90e11, c44_pa: 1.20e11 },
];

pub fn by_formula(formula: &str) -> Option<&'static HexagonalElastic> {
    TABLE.iter().find(|h| h.formula == formula)
}
