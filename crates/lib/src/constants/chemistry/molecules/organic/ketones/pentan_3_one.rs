use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C5H10O",
        name: "pentan-3-one",
        composition: &[(6, 5), (1, 10), (8, 1)],
        molar_mass: 86.134,
        category: "ketone",
        state_at_stp: "liquid",
        melting_point_k: Some(231.0),
        boiling_point_k: Some(375.0),
        density_g_cm3: Some(0.814),
    }
}
