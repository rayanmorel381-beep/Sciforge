use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C11H12N2O2",
        name: "tryptophan",
        composition: &[(6, 11), (1, 12), (7, 2), (8, 2)],
        molar_mass: 204.229,
        category: "amino_acid",
        state_at_stp: "solid",
        melting_point_k: Some(563.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.34),
    }
}
