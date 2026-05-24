use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H10O5",
        name: "ribose",
        composition: &[(6, 5), (1, 10), (8, 5)],
        molar_mass: 150.130,
        category: "monosaccharide",
        state_at_stp: "solid",
        melting_point_k: Some(363.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.660),
    }
}
