#[derive(Debug, Clone, Copy)]
pub struct CrystalElastic {
    pub formula: &'static str,
    pub c11_pa: f64,
    pub c12_pa: f64,
    pub c44_pa: f64,
    pub lattice: &'static str,
}

pub const TABLE: &[CrystalElastic] = &[
    CrystalElastic { formula: "Si",      c11_pa: 1.658e11, c12_pa: 6.39e10,  c44_pa: 7.96e10, lattice: "diamond_cubic" },
    CrystalElastic { formula: "Ge",      c11_pa: 1.292e11, c12_pa: 4.79e10,  c44_pa: 6.70e10, lattice: "diamond_cubic" },
    CrystalElastic { formula: "diamond", c11_pa: 1.076e12, c12_pa: 1.25e11,  c44_pa: 5.77e11, lattice: "diamond_cubic" },
    CrystalElastic { formula: "Cu",      c11_pa: 1.683e11, c12_pa: 1.221e11, c44_pa: 7.57e10, lattice: "fcc" },
    CrystalElastic { formula: "Al",      c11_pa: 1.067e11, c12_pa: 6.04e10,  c44_pa: 2.83e10, lattice: "fcc" },
    CrystalElastic { formula: "Au",      c11_pa: 1.924e11, c12_pa: 1.630e11, c44_pa: 4.20e10, lattice: "fcc" },
    CrystalElastic { formula: "Ag",      c11_pa: 1.240e11, c12_pa: 9.34e10,  c44_pa: 4.61e10, lattice: "fcc" },
    CrystalElastic { formula: "Ni",      c11_pa: 2.481e11, c12_pa: 1.549e11, c44_pa: 1.242e11,lattice: "fcc" },
    CrystalElastic { formula: "Fe",      c11_pa: 2.30e11,  c12_pa: 1.35e11,  c44_pa: 1.17e11, lattice: "bcc" },
    CrystalElastic { formula: "W",       c11_pa: 5.224e11, c12_pa: 2.044e11, c44_pa: 1.606e11,lattice: "bcc" },
    CrystalElastic { formula: "Mo",      c11_pa: 4.637e11, c12_pa: 1.578e11, c44_pa: 1.092e11,lattice: "bcc" },
    CrystalElastic { formula: "Cr",      c11_pa: 3.398e11, c12_pa: 5.86e10,  c44_pa: 9.90e10, lattice: "bcc" },
    CrystalElastic { formula: "NaCl",    c11_pa: 4.87e10,  c12_pa: 1.24e10,  c44_pa: 1.26e10, lattice: "fcc_b1" },
    CrystalElastic { formula: "MgO",     c11_pa: 2.97e11,  c12_pa: 9.50e10,  c44_pa: 1.56e11, lattice: "fcc_b1" },
    CrystalElastic { formula: "KCl",     c11_pa: 4.07e10,  c12_pa: 7.00e9,   c44_pa: 6.30e9,  lattice: "fcc_b1" },
];

pub fn by_formula(formula: &str) -> Option<&'static CrystalElastic> {
    TABLE.iter().find(|c| c.formula == formula)
}

pub fn anisotropy_zener(entry: &CrystalElastic) -> f64 {
    2.0 * entry.c44_pa / (entry.c11_pa - entry.c12_pa)
}
