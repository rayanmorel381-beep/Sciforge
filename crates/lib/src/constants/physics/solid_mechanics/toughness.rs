#[derive(Debug, Clone, Copy)]
pub struct FractureToughness {
    pub material: &'static str,
    pub k_ic_pa_sqrt_m: f64,
}

pub const TABLE: &[FractureToughness] = &[
    FractureToughness { material: "Fe",          k_ic_pa_sqrt_m:  50.0e6 },
    FractureToughness { material: "Cu",          k_ic_pa_sqrt_m: 100.0e6 },
    FractureToughness { material: "Al",          k_ic_pa_sqrt_m:  35.0e6 },
    FractureToughness { material: "Ti",          k_ic_pa_sqrt_m:  55.0e6 },
    FractureToughness { material: "Ni",          k_ic_pa_sqrt_m: 100.0e6 },
    FractureToughness { material: "AISI_1045",   k_ic_pa_sqrt_m:  50.0e6 },
    FractureToughness { material: "AISI_4140",   k_ic_pa_sqrt_m:  60.0e6 },
    FractureToughness { material: "AISI_316",    k_ic_pa_sqrt_m: 112.0e6 },
    FractureToughness { material: "AISI_304",    k_ic_pa_sqrt_m: 220.0e6 },
    FractureToughness { material: "Al_6061_T6",  k_ic_pa_sqrt_m:  29.0e6 },
    FractureToughness { material: "Al_7075_T6",  k_ic_pa_sqrt_m:  24.0e6 },
    FractureToughness { material: "Al_2024_T3",  k_ic_pa_sqrt_m:  44.0e6 },
    FractureToughness { material: "Ti_6Al_4V",   k_ic_pa_sqrt_m:  55.0e6 },
    FractureToughness { material: "Inconel_718", k_ic_pa_sqrt_m:  96.0e6 },
    FractureToughness { material: "SiO2_glass",  k_ic_pa_sqrt_m:   0.7e6 },
    FractureToughness { material: "Al2O3",       k_ic_pa_sqrt_m:   3.5e6 },
    FractureToughness { material: "Si3N4",       k_ic_pa_sqrt_m:   6.0e6 },
    FractureToughness { material: "SiC",         k_ic_pa_sqrt_m:   3.5e6 },
    FractureToughness { material: "ZrO2",        k_ic_pa_sqrt_m:  10.0e6 },
    FractureToughness { material: "WC",          k_ic_pa_sqrt_m:  18.0e6 },
    FractureToughness { material: "concrete",    k_ic_pa_sqrt_m:   1.0e6 },
    FractureToughness { material: "wood_pine",   k_ic_pa_sqrt_m:   2.0e6 },
    FractureToughness { material: "PE",          k_ic_pa_sqrt_m:   2.0e6 },
    FractureToughness { material: "PP",          k_ic_pa_sqrt_m:   3.5e6 },
    FractureToughness { material: "PS",          k_ic_pa_sqrt_m:   1.0e6 },
    FractureToughness { material: "PVC",         k_ic_pa_sqrt_m:   3.5e6 },
    FractureToughness { material: "PMMA",        k_ic_pa_sqrt_m:   1.0e6 },
    FractureToughness { material: "PET",         k_ic_pa_sqrt_m:   5.0e6 },
];

pub fn by_material(material: &str) -> Option<&'static FractureToughness> {
    TABLE.iter().find(|t| t.material == material)
}
