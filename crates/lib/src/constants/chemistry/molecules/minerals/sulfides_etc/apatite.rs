use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Ca5(PO4)3F",
        name: "fluorapatite",
        composition: &[(8, 12), (9, 1), (15, 3), (20, 5)],
        molar_mass: 504.302,
        category: "phosphate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(3.200),
    }
}
