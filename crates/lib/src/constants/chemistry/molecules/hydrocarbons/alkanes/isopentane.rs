use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H12",
        name: "isopentane",
        composition: &[(6, 5), (1, 12)],
        molar_mass: 72.149,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(113.3),
        boiling_point_k: Some(300.99),
        density_g_cm3: Some(0.620),
    }
}
