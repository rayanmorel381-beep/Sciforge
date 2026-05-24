#[derive(Debug, Clone, Copy)]
pub struct YieldStrength {
    pub material: &'static str,
    pub yield_pa: f64,
    pub ultimate_pa: f64,
    pub elongation_pct: f64,
}

pub const TABLE: &[YieldStrength] = &[
    YieldStrength { material: "Fe",         yield_pa: 1.65e8, ultimate_pa: 3.50e8, elongation_pct: 30.0 },
    YieldStrength { material: "Cu",         yield_pa: 0.70e8, ultimate_pa: 2.20e8, elongation_pct: 45.0 },
    YieldStrength { material: "Al",         yield_pa: 1.10e7, ultimate_pa: 9.00e7, elongation_pct: 50.0 },
    YieldStrength { material: "Au",         yield_pa: 1.00e8, ultimate_pa: 2.20e8, elongation_pct: 45.0 },
    YieldStrength { material: "Ag",         yield_pa: 5.50e7, ultimate_pa: 1.70e8, elongation_pct: 50.0 },
    YieldStrength { material: "Ni",         yield_pa: 1.40e8, ultimate_pa: 4.95e8, elongation_pct: 40.0 },
    YieldStrength { material: "Ti",         yield_pa: 1.40e8, ultimate_pa: 2.20e8, elongation_pct: 54.0 },
    YieldStrength { material: "Mg",         yield_pa: 2.10e7, ultimate_pa: 9.00e7, elongation_pct: 11.0 },
    YieldStrength { material: "W",          yield_pa: 7.50e8, ultimate_pa: 1.51e9, elongation_pct:  4.0 },
    YieldStrength { material: "Pb",         yield_pa: 5.00e6, ultimate_pa: 1.20e7, elongation_pct: 50.0 },
    YieldStrength { material: "Zn",         yield_pa: 1.50e8, ultimate_pa: 2.00e8, elongation_pct: 65.0 },
    YieldStrength { material: "Sn",         yield_pa: 1.10e7, ultimate_pa: 2.20e7, elongation_pct: 55.0 },
    YieldStrength { material: "AISI_1020",  yield_pa: 3.50e8, ultimate_pa: 4.50e8, elongation_pct: 25.0 },
    YieldStrength { material: "AISI_1045",  yield_pa: 5.30e8, ultimate_pa: 6.25e8, elongation_pct: 16.0 },
    YieldStrength { material: "AISI_4140",  yield_pa: 6.55e8, ultimate_pa: 1.02e9, elongation_pct: 17.7 },
    YieldStrength { material: "AISI_316",   yield_pa: 2.05e8, ultimate_pa: 5.15e8, elongation_pct: 40.0 },
    YieldStrength { material: "AISI_304",   yield_pa: 2.15e8, ultimate_pa: 5.05e8, elongation_pct: 70.0 },
    YieldStrength { material: "Al_6061_T6", yield_pa: 2.76e8, ultimate_pa: 3.10e8, elongation_pct: 12.0 },
    YieldStrength { material: "Al_7075_T6", yield_pa: 5.03e8, ultimate_pa: 5.72e8, elongation_pct: 11.0 },
    YieldStrength { material: "Al_2024_T3", yield_pa: 3.45e8, ultimate_pa: 4.83e8, elongation_pct: 18.0 },
    YieldStrength { material: "Ti_6Al_4V",  yield_pa: 8.80e8, ultimate_pa: 9.50e8, elongation_pct: 14.0 },
    YieldStrength { material: "Inconel_718",yield_pa: 1.03e9, ultimate_pa: 1.24e9, elongation_pct: 12.0 },
    YieldStrength { material: "Cu_brass",   yield_pa: 1.24e8, ultimate_pa: 3.30e8, elongation_pct: 45.0 },
    YieldStrength { material: "Cu_bronze",  yield_pa: 1.50e8, ultimate_pa: 3.50e8, elongation_pct: 30.0 },
    YieldStrength { material: "concrete",   yield_pa: 0.0,    ultimate_pa: 4.00e7, elongation_pct:  0.4 },
    YieldStrength { material: "wood_pine",  yield_pa: 4.00e7, ultimate_pa: 5.00e7, elongation_pct:  1.0 },
    YieldStrength { material: "PE",         yield_pa: 2.60e7, ultimate_pa: 3.30e7, elongation_pct: 600.0 },
    YieldStrength { material: "PP",         yield_pa: 3.50e7, ultimate_pa: 4.00e7, elongation_pct: 200.0 },
    YieldStrength { material: "PS",         yield_pa: 4.50e7, ultimate_pa: 5.00e7, elongation_pct:  3.0 },
    YieldStrength { material: "PVC",        yield_pa: 5.50e7, ultimate_pa: 6.50e7, elongation_pct: 25.0 },
    YieldStrength { material: "PET",        yield_pa: 5.50e7, ultimate_pa: 7.50e7, elongation_pct: 70.0 },
    YieldStrength { material: "PMMA",       yield_pa: 7.20e7, ultimate_pa: 7.50e7, elongation_pct:  4.0 },
];

pub fn by_material(material: &str) -> Option<&'static YieldStrength> {
    TABLE.iter().find(|y| y.material == material)
}
