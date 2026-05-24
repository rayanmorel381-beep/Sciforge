use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H14N4O2",
        name: "arginine",
        composition: &[(6, 6), (1, 14), (7, 4), (8, 2)],
        molar_mass: 174.201,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(517.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.1),
    }
}
