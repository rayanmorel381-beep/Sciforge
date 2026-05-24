use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H14N2O2",
        name: "lysine",
        composition: &[(6, 6), (1, 14), (7, 2), (8, 2)],
        molar_mass: 146.188,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(497.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.125),
    }
}
