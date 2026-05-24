#[derive(Debug, Clone, Copy)]
pub struct Sellmeier {
    pub formula: &'static str,
    pub b1: f64,
    pub c1_um2: f64,
    pub b2: f64,
    pub c2_um2: f64,
    pub b3: f64,
    pub c3_um2: f64,
    pub valid_range_um: (f64, f64),
}

pub const TABLE: &[Sellmeier] = &[
    Sellmeier { formula: "SiO2",  b1: 0.6961663,  c1_um2: 0.004679148,    b2: 0.4079426,  c2_um2: 0.013512063,    b3: 0.8974794, c3_um2: 97.93400025,    valid_range_um: (0.21, 6.7) },
    Sellmeier { formula: "BK7",   b1: 1.03961212, c1_um2: 0.00600069867,  b2: 0.231792344,c2_um2: 0.0200179144,   b3: 1.01046945,c3_um2: 103.560653,     valid_range_um: (0.30, 2.5) },
    Sellmeier { formula: "SF11",  b1: 1.73759695, c1_um2: 0.013188707,    b2: 0.313747346,c2_um2: 0.0623068142,   b3: 1.89878101,c3_um2: 155.23629,      valid_range_um: (0.37, 2.5) },
    Sellmeier { formula: "CaF2",  b1: 0.5675888,  c1_um2: 0.002526432,    b2: 0.4710914,  c2_um2: 0.010078334,    b3: 3.8484723, c3_um2: 1200.55609,  valid_range_um: (0.23, 9.7) },
    Sellmeier { formula: "MgF2",  b1: 0.48755108, c1_um2: 0.001882178,    b2: 0.39875031, c2_um2: 0.008951889,    b3: 2.3120353, c3_um2: 566.13567,      valid_range_um: (0.20, 7.0) },
    Sellmeier { formula: "Al2O3", b1: 1.4313493,  c1_um2: 0.005279924,    b2: 0.65054713, c2_um2: 0.014238828,    b3: 5.3414021, c3_um2: 325.01751,      valid_range_um: (0.20, 5.5) },
    Sellmeier { formula: "Si",    b1: 10.6684293, c1_um2: 0.090912191,    b2: 0.0030434748,c2_um2: 1.287660,      b3: 1.54133408,c3_um2: 1218816.0,      valid_range_um: (1.20, 14.0) },
    Sellmeier { formula: "Ge",    b1: 6.72880,    c1_um2: 0.44105,        b2: 0.21307,    c2_um2: 3.8261,         b3: 0.0,        c3_um2: 1.0,           valid_range_um: (2.00, 14.0) },
    Sellmeier { formula: "H2O",   b1: 0.5684027565,c1_um2: 0.005101830,   b2: 0.1726177391,c2_um2: 0.018211539,   b3: 0.02086189578,c3_um2: 0.026207223, valid_range_um: (0.20, 1.10) },
    Sellmeier { formula: "NaCl",  b1: 1.0079,     c1_um2: 0.001666517,    b2: 0.28200,    c2_um2: 0.026925529,    b3: 3.1747,     c3_um2: 629.257225,    valid_range_um: (0.20, 30.0) },
];

pub fn by_formula(formula: &str) -> Option<&'static Sellmeier> {
    TABLE.iter().find(|s| s.formula == formula)
}
