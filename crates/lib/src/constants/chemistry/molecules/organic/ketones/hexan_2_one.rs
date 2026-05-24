use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C6H12O",
        name: "hexan-2-one",
        composition: &[(6, 6), (1, 12), (8, 1)],
        molar_mass: 100.161,
        category: "ketone",
        state_at_stp: "liquid",
        melting_point_k: Some(217.0),
        boiling_point_k: Some(400.0),
        density_g_cm3: Some(0.812),
    }
}
